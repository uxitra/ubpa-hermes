mod check_email;
mod check_pdf;
mod close_popup;
mod load;
mod login;
mod privacy;
mod templates;
mod view_status;

use crate::close_popup::close_modal;
use crate::load::load;
use crate::login::login;
use crate::privacy::privacy;
use crate::view_status::view_status;
use actix_files as fs;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::web::PayloadConfig;
use actix_web::{App, HttpServer, web};
use sqlx::Sqlite;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;

// Refactor the html kind so that index.html only is full of containers and containers get inplaced by templates so everything is structured clearly and reusable

// <div> // only containers in index.html
// |
// |
// Templates place their html into the containers.
// templates that have placed a new body before now just insert into a div and can now still print into other divs

// build like main page but instead of containing main page form contain template after request

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_path = "sqlite:data/data.db";

    std::fs::create_dir_all("./data")?;

    let db_exists = Sqlite::database_exists(db_path).await.unwrap();

    if !db_exists {
        match Sqlite::create_database(db_path).await {
            Ok(_) => println!("Created database"),
            Err(e) => panic!("{}", e),
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path)
        .await
        .expect("Failed to create conection to the database");

    if !db_exists {
        println!("Database not found, creating new one...");
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();
        println!("Table 'users' created.");
    } else {
        println!("Database already exists, skipping table creation.");
    }

    std::fs::create_dir_all("./upload")?;

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            // Set a payload so people cant upload extremely big files
            .app_data(PayloadConfig::new(10 * 1024 * 1024))
            // Set the location for temporary files
            .app_data(TempFileConfig::default().directory("./upload"))
            .app_data(web::Data::new(pool.clone()))
            // Serve everything under /static/
            .service(fs::Files::new("/html/", "./static/html/").prefer_utf8(true))
            .service(fs::Files::new("/js/", "./static/js/").show_files_listing())
            // Route for HTMX dynamic response
            .route("/load", web::post().to(load))
            .route("/privacy", web::get().to(privacy))
            .route("/close-modal", web::get().to(close_modal))
            .route("/login", web::get().to(login))
            .route("/view-status", web::post().to(view_status))
            .default_service(
                web::get()
                    .to(|| async { fs::NamedFile::open_async("./static/html/index.html").await }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
