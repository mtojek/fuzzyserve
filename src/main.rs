use std::path::PathBuf;

use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use clap::Parser;

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
    web::Redirect::to("/files/src/main.rs").see_other()
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
