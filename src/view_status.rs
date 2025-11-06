use crate::templates::staus_template::StatusTemplate;
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
    let row = sqlx::query("SELECT 1 FROM users WHERE key = ? AND value = ?")
        .bind(&form.token)
        .bind(&form.email)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|e| {
            println!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    println!("loaded and got data from DB");

    if row.is_some() {
        // Found
        println!("Found you");
        Ok(StatusTemplate { error: "" })
    } else {
        println!("Fake!!!!");
        // Not found — common case, just respond accordingly
        Ok(StatusTemplate {
            error: "User not found",
        })
    }
}
