mod check_email;
mod check_pdf;
mod close_popup;
mod load;
mod privacy;
mod templates;

use crate::close_popup::close_modal;
use crate::load::load;
use crate::privacy::privacy;
use actix_files as fs;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::web::PayloadConfig;
use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");

    std::fs::create_dir_all("./upload")?;

    HttpServer::new(|| {
        App::new()
            // Set a payload so people cant upload extremely big files
            .app_data(PayloadConfig::new(10 * 1024 * 1024))
            // Set the location for temporary files
            .app_data(TempFileConfig::default().directory("./upload"))
            // Serve everything under /static/
            .service(fs::Files::new("/html/", "./static/html/").prefer_utf8(true))
            .service(fs::Files::new("/js/", "./static/js/").show_files_listing())
            // Route for HTMX dynamic response
            .route("/load", web::post().to(load))
            .route("/privacy", web::get().to(privacy))
            .route("/close-modal", web::get().to(close_modal))
            .default_service(
                web::get()
                    .to(|| async { fs::NamedFile::open_async("./static/html/index.html").await }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
