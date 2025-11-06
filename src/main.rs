mod background_worker;
mod change_state;
mod check_email;
mod check_pdf;
mod close_popup;
mod dashboard;
mod home;
mod load;
mod load_config;
mod login;
mod login_admin;
mod privacy;
mod remove_state;
mod send_email;
mod state;
mod templates;
mod view_status;
mod view_status_admin;

use crate::background_worker::background_worker;
use crate::change_state::change_state;
use crate::close_popup::close_modal;
use crate::home::home;
use crate::load::load;
use crate::login::login;
use crate::login_admin::login_admin;
use crate::privacy::privacy;
use crate::remove_state::remove_state;
use crate::view_status::view_status;
use crate::view_status_admin::view_status_admin;
use actix_files as fs;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::web::PayloadConfig;
use actix_web::{App, HttpServer, web};
use sqlx::Sqlite;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_path = "sqlite:data/data.db";

    // Create data directory if not existing
    std::fs::create_dir_all("./data")?;

    let config_path = std::path::Path::new("config.json");

    // Check if config file does exist
    if !config_path.exists() {
        panic!(
            r#"The Program needs a config file for sending emails if the file doesnt exist the program will panic later! 
            Please create an config file (hint: there is a example config file)"#
        )
    }

    let config = load_config::load_config().await;

    // Check if enviroment variables exist
    match std::env::var("ADMIN_USERNAME") {
        Ok(_) => {}
        Err(e) => {
            eprint!("{}", e);
            panic!(
                "Could not find enviroment variable 'ADMIN_USERNAME' which is crucial for the admin dashboard"
            )
        }
    }

    match std::env::var("ADMIN_PASSWORD") {
        Ok(_) => {}
        Err(e) => {
            eprint!("{}", e);
            panic!(
                "Could not find enviroment variable 'ADMIN_PASSWORD' which is crucial for the admin dashboard"
            )
        }
    }

    // Check if the database exists
    let db_exists = Sqlite::database_exists(db_path).await.unwrap();

    // If the database wasnt created create it
    if !db_exists {
        match Sqlite::create_database(db_path).await {
            Ok(_) => println!("Created database"),
            Err(e) => panic!("{}", e),
        }
    }

    // Create a connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path)
        .await
        .expect("Failed to create conection to the database");

    // If this is the first time the db starts create table
    if !db_exists {
        println!("Database not found, creating new one...");
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS applicants (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                state INT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
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

    // Create an upload directory for uploaded files
    std::fs::create_dir_all("./upload")?;

    // Spawn the background worker
    tokio::spawn(background_worker(pool.clone()));

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            // Set a payload so people cant upload extremely big files
            .app_data(PayloadConfig::new(10 * 1024 * 1024))
            // Set the location for temporary files
            .app_data(TempFileConfig::default().directory("./upload"))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            // Serve everything under /static/
            .service(fs::Files::new("/html/", "./static/html/").prefer_utf8(true))
            .service(fs::Files::new("/js/", "./static/js/").show_files_listing())
            // Route for HTMX dynamic response
            .route("/load", web::post().to(load))
            .route("/privacy", web::get().to(privacy))
            .route("/close-modal", web::get().to(close_modal))
            .route("/login", web::get().to(login))
            .route("/view-status", web::post().to(view_status))
            .route("/home", web::get().to(home))
            .route("/login_admin", web::get().to(login_admin))
            .route("/view-status-admin", web::post().to(view_status_admin))
            .route("/change_state", web::post().to(change_state))
            .route("/remove_state", web::post().to(remove_state))
            .default_service(
                web::get()
                    .to(|| async { fs::NamedFile::open_async("./static/html/index.html").await }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
