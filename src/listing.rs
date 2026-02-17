use percent_encoding::{CONTROLS, utf8_percent_encode};
use std::fmt::Write;
use std::io;
use std::path::Path;
use v_htmlescape::escape as escape_html_entity;

use actix_files::Directory;
use actix_web::{HttpRequest, HttpResponse, dev::ServiceResponse};

macro_rules! encode_file_url {
    ($path:ident) => {
        utf8_percent_encode(&$path, CONTROLS)
    };
}

macro_rules! encode_file_name {
    ($entry:ident) => {
        escape_html_entity(
            &$entry
                .file_name()
                .to_string_lossy(),
        )
    };
}

pub fn dirs_first(dir: &Directory, req: &HttpRequest) -> Result<ServiceResponse, io::Error> {
    let index_of = format!("Index of {}", req.path());
    let mut body = String::new();
    let base = Path::new(req.path());

    let mut entries: Vec<_> = dir
        .path
        .read_dir()?
        .filter(|e| dir.is_visible(e))
        .filter_map(|e| e.ok())
        .collect();

    // sort: dirs first, by name second
    entries.sort_by(|a, b| {
        let a_is_dir = a
            .metadata()
            .map(|m| m.is_dir())
            .unwrap_or(false);
        let b_is_dir = b
            .metadata()
            .map(|m| m.is_dir())
            .unwrap_or(false);
        b_is_dir
            .cmp(&a_is_dir)
            .then_with(|| {
                a.file_name()
                    .cmp(&b.file_name())
            })
    });

    for entry in entries {
        let p = match entry
            .path()
            .strip_prefix(&dir.path)
        {
            Ok(p) if cfg!(windows) => base
                .join(p)
                .to_string_lossy()
                .replace('\\', "/"),
            Ok(p) => base
                .join(p)
                .to_string_lossy()
                .into_owned(),
            Err(_) => continue,
        };

        // if file is a directory, add '/' to the end of the name
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_dir() {
                let _ = write!(body, "<li><a href=\"{}\">{}/</a></li>", encode_file_url!(p), encode_file_name!(entry),);
            } else {
                let _ = write!(body, "<li><a href=\"{}\">{}</a></li>", encode_file_url!(p), encode_file_name!(entry),);
            }
        }
    }

    let html = format!(
        "<html>\
         <head><title>{}</title></head>\
         <body><h1>{}</h1>\
         <ul>\
         {}\
         </ul></body>\n</html>",
        index_of, index_of, body
    );
    Ok(ServiceResponse::new(
        req.clone(),
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
    ))
}
