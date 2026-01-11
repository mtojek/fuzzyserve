use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool},
    thread,
    time::Duration,
};

use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use clap::Parser;
use notify::{Watcher, recommended_watcher};
use parking_lot::RwLock;
use strsim::normalized_levenshtein;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "fuzzyserve")]
#[command(about = "Fuzzy media file server")]
struct Args {
    #[arg(short, long, default_value = ".")]
    media_root: PathBuf,

    #[arg(short, long, default_value_t = 7666)]
    port: u16,

    #[arg(short, long, default_value = "0.0.0.0")]
    addr: String,
}

#[derive(Clone)]
struct AppState {
    files: Arc<RwLock<Vec<String>>>,
}

async fn index_handler() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn download_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let query = path.into_inner();
    let files = data.files.read();

    match find_best_match(&query, &files) {
        Some(relative_path) => {
            let location = format!("/files/{}", urlencoding::encode(&relative_path));
            HttpResponse::SeeOther()
                .insert_header(("Location", location))
                .finish()
        }
        None => HttpResponse::NotFound().body("File not found"),
    }
}

const MEDIA_EXTENSIONS: &[&str] = &["mkv", "mp4", "avi", "mov"];

fn scan_media_files(root: &Path) -> Vec<String> {
    let files: Vec<String> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| MEDIA_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
        })
        .filter_map(|e| {
            e.path()
                .strip_prefix(root)
                .ok()
                .map(|p| p.to_string_lossy().into_owned())
        })
        .collect();

    println!("Found {} files", files.len());
    for f in &files {
        println!("  {}", f);
    }
    files
}

fn find_best_match<'a>(query: &str, files: &'a [String]) -> Option<&'a str> {
    let query_file_stem = normalize(
        Path::new(&query)
            .file_stem()
            .and_then(|f| f.to_str())
            .unwrap_or(query),
    );

    files
        .iter()
        .filter_map(|path| {
            let file_stem = &normalize(Path::new(path).file_stem()?.to_str()?);
            let score = normalized_levenshtein(&query_file_stem, &file_stem);
            Some((path.as_str(), score))
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .filter(|(_, score)| *score > 0.3)
        .map(|(path, score)| {
            dbg!(&path, score);
            (path, score)
        })
        .map(|(path, _)| path)
}

fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

fn start_watcher(
    media_root: PathBuf,
    files: Arc<RwLock<Vec<String>>>,
    stop: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    let media_root_clone = media_root.clone();

    thread::spawn(move || {
        let mut watcher = recommended_watcher(move |res: Result<notify::Event, _>| match res {
            Ok(event) => {
                let is_media = event.paths.iter().any(|p| {
                    p.extension().and_then(|ext| ext.to_str()).is_some_and(|f| {
                        MEDIA_EXTENSIONS.contains(&f.to_ascii_lowercase().as_str())
                    })
                });
                if !is_media {
                    return;
                }

                dbg!(&event);
                let new_files = scan_media_files(&media_root_clone);
                *files.write() = new_files;
                println!("Reloaded files");
            }
            Err(e) => {
                eprintln!("Watch error: {}", e)
            }
        })
        .unwrap();

        watcher
            .watch(&media_root, notify::RecursiveMode::Recursive)
            .unwrap();

        while !stop.load(std::sync::atomic::Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(100));
        }
        println!("Watcher stopped");
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let media_root = args.media_root;
    let files: Arc<parking_lot::lock_api::RwLock<parking_lot::RawRwLock, Vec<String>>> =
        Arc::new(RwLock::new(scan_media_files(&media_root)));

    println!("Media root: {}", media_root.display());
    println!("Initial scan: {} files", files.read().len());

    let stop = Arc::new(AtomicBool::new(false));
    let watcher_handle = start_watcher(media_root.clone(), files.clone(), stop.clone());

    println!(
        "Serving files from {} on http://{}:{}",
        media_root.display(),
        args.addr,
        args.port
    );

    let state: AppState = AppState {
        files: files.clone(),
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(index_handler))
            .route("/get/{query:.*}", web::get().to(download_handler))
            .service(Files::new("/files", &media_root).show_files_listing())
    })
    .bind((args.addr, args.port))?
    .run();
    let server_handle = server.handle();

    let stop_clone = stop.clone();
    ctrlc::set_handler(move || {
        println!("Shutting down");
        stop_clone.store(true, std::sync::atomic::Ordering::Relaxed);
        let _ = server_handle.stop(true);
    })
    .unwrap();

    server.await?;
    watcher_handle.join().unwrap();
    println!("Goodbye!");

    Ok(())
}
