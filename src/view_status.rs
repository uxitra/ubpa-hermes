use crate::templates::staus_template::StatusTemplate;
use sqlx::Row;
use sqlx::SqlitePool;

#[derive(serde::Deserialize, Debug)]
pub struct LoginForm {
    pub token: String,
    pub email: String,
}

pub async fn view_status(
    form: actix_web::web::Form<LoginForm>,
    pool: actix_web::web::Data<SqlitePool>,
) -> Result<impl actix_web::Responder, actix_web::Error> {
    println!("{}", form.email);
    println!("{}", form.token);
    let row = sqlx::query("SELECT key, value, state FROM applicants WHERE key = ? AND value = ?")
        .bind(&form.token)
        .bind(&form.email)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|e| {
            println!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    println!("loaded and got data from DB");

    if let Some(row) = row {
        // Found
        let state: i32 = row.get("state");

        let displayed_state = match state {
            1 => "Send",
            2 => "In Progress",
            _ => "Unknown state",
        };
        println!("state is: {}", state);
        println!("Found you");
        Ok(StatusTemplate {
            error: "",
            state: displayed_state,
        })
    } else {
        println!("Fake!!!!");
        Ok(StatusTemplate {
            error: "User not found",
            state: "",
        })
    }
}
