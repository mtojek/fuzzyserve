use std::path::{Path, PathBuf};

use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use clap::Parser;
use strsim::normalized_levenshtein;

#[derive(Parser)]
#[command(name = "fuzzyserve")]
#[command(name = "Fuzzy media file server")]
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

    match find_best_match(query, files) {
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
    vec![] // TODO
}

fn find_best_match(query: String, files: Vec<String>) -> Option<String> {
    let normalized_query = normalize(query);

    files
        .into_iter()
        .map(|path| {
            let normalized_name = normalize(path);
            let score = normalized_levenshtein(&normalized_query, &normalized_name);
            (normalized_name, score)
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .filter(|(_, score)| *score > 0.3)
        .map(|(path, _)| path)
}

fn normalize(s: String) -> String {
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

    let state = AppState {
        media_root: args.media_root.clone(),
    };

    let media_root = args.media_root.clone();
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
