use actix_web::{Responder, web};
use sqlx::Row;
use sqlx::SqlitePool;

use crate::change_state::ChangeStateForm;
use crate::dashboard;
use crate::send_email::EmailConfig;
use crate::send_email::send_email;
use crate::templates::admin_dashboard::AdminDashboardTemplate;

/// Removes an user from the db
pub async fn remove_state(
    form: web::Form<ChangeStateForm>,
    pool: actix_web::web::Data<SqlitePool>,
    config: actix_web::web::Data<EmailConfig>,
) -> Result<impl Responder, actix_web::Error> {
    let key = &form.key;

    // Get the value (email) to send a email to the user saying their application was deleted
    let row = sqlx::query("SELECT value FROM applicants WHERE key = ?")
        .bind(key)
        .fetch_optional(pool.get_ref())
        .await;

    match row {
        Ok(Some(record)) => {
            let value: String = record.get("value");
            // Create the email
            send_email(value, config.get_ref());
        }
        Ok(None) => {}
        Err(err) => {
            eprintln!("Database error: {}", err);
        }
    }

    // Delete the user from the db
    let result = sqlx::query("DELETE FROM applicants WHERE key = ?")
        .bind(key)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => dashboard::dashboard(pool.as_ref()).await,
        Err(err) => {
            eprintln!("Database error: {}", err);
            Ok(AdminDashboardTemplate {
                applicants: Vec::new(),
            })
        }
    }
}
