mod change_state;
mod check_email;
mod check_pdf;
mod close_popup;
mod dashboard;
mod home;
mod load;
mod login;
mod login_admin;
mod privacy;
mod remove_state;
mod send_email;
mod state;
mod templates;
mod view_status;
mod view_status_admin;

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
use chrono::Local;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Row, Sqlite, SqlitePool};
use tokio::time::{Duration, interval};

/// Runs every 24 hours on a background thread
pub async fn background_worker(pool: SqlitePool) {
    let mut ticker = interval(Duration::from_hours(24));

    loop {
        ticker.tick().await;
        println!("Running scheduled task...");

        // Get current time as UTC
        let current_time = Local::now().naive_local();

        // Get the rows
        let rows = sqlx::query("SELECT key, value, state, created_at FROM applicants")
            .fetch_all(&pool)
            .await
            .unwrap();

        for row in rows {
            let key: String = row.get("key");
            let created_at: String = row.get("created_at");
            let state: i32 = row.get("state");
            let user_created_at: chrono::NaiveDateTime =
                chrono::NaiveDateTime::parse_from_str(created_at.as_str(), "%Y-%m-%d %H:%M:%S")
                    .unwrap();

            let duration_in_db = current_time - user_created_at;

            // If the duration equals one day and the state is 'fresh' it will be set to 'old'
            if duration_in_db.num_days() == 1 && state == 1 {
                let new_state = state + 1;

                // Update the state in the db
                sqlx::query("UPDATE applicants SET state = ? WHERE key = ?")
                    .bind(new_state)
                    .bind(key)
                    .execute(&pool)
                    .await
                    .unwrap();
            }
        }
    }
}

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
