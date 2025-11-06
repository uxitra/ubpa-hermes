mod check_email;
mod check_pdf;
mod close_popup;
mod home;
mod load;
mod login;
mod login_admin;
mod privacy;
mod state;
mod templates;
mod view_status;
mod view_status_admin;

use crate::close_popup::close_modal;
use crate::home::home;
use crate::load::load;
use crate::login::login;
use crate::login_admin::login_admin;
use crate::privacy::privacy;
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

pub async fn background_worker(pool: SqlitePool) {
    let mut ticker = interval(Duration::from_hours(24));

    loop {
        ticker.tick().await;
        println!("Running scheduled task...");
        let current_time = Local::now().naive_local();

        let rows = sqlx::query("SELECT key, value, state, created_at FROM users")
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

            if duration_in_db.num_days() == 1 && state == 1 {
                let new_state = state + 1;

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

    std::fs::create_dir_all("./upload")?;

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
            .default_service(
                web::get()
                    .to(|| async { fs::NamedFile::open_async("./static/html/index.html").await }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
