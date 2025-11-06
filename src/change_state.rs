use crate::dashboard;
use crate::send_email;
use actix_web::{Responder, web};
use serde::Deserialize;
use sqlx::Row;
use sqlx::SqlitePool;

use crate::templates::admin_dashboard::AdminDashboardTemplate;

#[derive(Deserialize)]
pub struct ChangeStateForm {
    pub key: String,
}

/// Change the state of an applicant
pub async fn change_state(
    form: web::Form<ChangeStateForm>,
    pool: actix_web::web::Data<SqlitePool>,
) -> Result<impl Responder, actix_web::Error> {
    let key = &form.key;

    // get the state and the email
    let row = sqlx::query("SELECT state, value FROM applicants WHERE key = ?")
        .bind(key)
        .fetch_optional(pool.get_ref())
        .await;

    match row {
        Ok(Some(row)) => {
            let state: i32 = row.get("state");
            let value: String = row.get("value");

            // Send an email to the email we got from the db
            send_email::send_email(value);

            // Change the state
            if state == 1 {
                let _result = sqlx::query("UPDATE applicants SET state = ? WHERE key = ?")
                    .bind(2)
                    .bind(key)
                    .execute(pool.get_ref())
                    .await;
            } else if state == 2 {
                let _result = sqlx::query("UPDATE applicants SET state = ? WHERE key = ?")
                    .bind(1)
                    .bind(key)
                    .execute(pool.get_ref())
                    .await;
            }
        }
        Ok(None) => {
            return Ok(AdminDashboardTemplate {
                applicants: Vec::new(),
            });
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Ok(AdminDashboardTemplate {
                applicants: Vec::new(),
            });
        }
    }

    // Return the admin dashboard that is now recalculated
    dashboard::dashboard(pool.as_ref()).await
}
