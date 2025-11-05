use sqlx::Row;
use sqlx::SqlitePool;
use std::env;

use crate::templates::admin_dashboard::AdminDashboardTemplate;

#[derive(serde::Deserialize, Debug)]
pub struct AdminLoginForm {
    pub password: String,
    pub username: String,
}

pub async fn view_status_admin(
    form: actix_web::web::Form<AdminLoginForm>,
    pool: actix_web::web::Data<SqlitePool>,
) -> Result<impl actix_web::Responder, actix_web::Error> {
    let username =
        env::var("ADMIN_USERNAME").expect("Failed to get enviroment variable 'ADMIN_USERNAME' ");
    let password =
        env::var("ADMIN_PASSWORD").expect("Failed to get enviroment variable 'ADMIN_PASSWORD' ");

    let mut users: Vec<String> = Vec::new();

    if form.username == username && form.password == password {
        let pool = pool.get_ref();

        let rows = sqlx::query("SELECT key, value FROM users")
            .fetch_all(pool)
            .await
            .unwrap();

        for row in rows {
            let key: String = row.get("key");
            let value: String = row.get("value");
            let user = format!("{},{}", key, value);
            users.push(user);
        }

        return Ok(AdminDashboardTemplate { applicants: users });
    }
    Ok(AdminDashboardTemplate {
        applicants: Vec::new(),
    })
}
