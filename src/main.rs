use std::path::{Path, PathBuf};

use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use clap::Parser;
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
    media_root: PathBuf,
}

async fn index_handler() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn download_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let query = path.into_inner();
    let files = scan_media_files(&data.media_root);

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
    WalkDir::new(root)
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
        .collect()
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!(
        "Serving files from {} on http://{}:{}",
        args.media_root.display(),
        args.addr,
        args.port
    );

    let media_root = args.media_root;
    let state = AppState {
        media_root: media_root.clone(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(index_handler))
            .route("/get/{query:.*}", web::get().to(download_handler))
            .service(Files::new("/files", &media_root).show_files_listing())
    })
    .bind((args.addr, args.port))?
    .run()
    .await
}
