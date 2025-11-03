use actix_files as fs;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::web::PayloadConfig;
use actix_web::{App, HttpServer, web};

mod check_email;
mod check_pdf;
mod close_popup;
mod load;
mod privacy;
mod templates;
use crate::close_popup::close_modal;
use crate::load::load;
use crate::privacy::privacy;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");

    std::fs::create_dir_all("./upload")?;

    HttpServer::new(|| {
        App::new()
            .app_data(PayloadConfig::new(10 * 1024 * 1024))
            .app_data(TempFileConfig::default().directory("./upload"))
            // Serve everything under /static/
            .service(fs::Files::new("/html/", "./static/html/").prefer_utf8(true))
            .service(fs::Files::new("/js/", "./static/js/").show_files_listing())
            //.service(fs::Files::new("/css/", "./static/css").prefer_utf8(true))
            //.service(fs::Files::new("/static", "./static").prefer_utf8(true))
            // Route for HTMX dynamic response
            // Default route -> serve the main HTML
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
