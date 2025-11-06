use sqlx::Row;
use sqlx::SqlitePool;

use crate::state::State;
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
    /*let username =
        std::env::var("ADMIN_USERNAME").expect("Failed to get enviroment variable 'ADMIN_USERNAME' ");
    let password =
        std::env::var("ADMIN_PASSWORD").expect("Failed to get enviroment variable 'ADMIN_PASSWORD' ");*/

    let mut users: Vec<String> = Vec::new();

    if form.username == "admin" && form.password == "12345" {
        let pool = pool.get_ref();

        let rows = sqlx::query("SELECT key, value, state, created_at FROM aplicants")
            .fetch_all(pool)
            .await
            .unwrap();

        for row in rows {
            let key: String = row.get("key");
            let value: String = row.get("value");
            let state: i32 = row.get("state");
            let created_at: String = row.get("created_at");

            let user_created_at: chrono::NaiveDateTime =
                chrono::NaiveDateTime::parse_from_str(created_at.as_str(), "%Y-%m-%d %H:%M:%S")
                    .unwrap();

            let real_state = match state {
                1 => State::Fresh,
                2 => State::Old,
                _ => State::None,
            };

            let user = format!(
                "{},  {}  state: {},  created at: {}",
                key, value, real_state, user_created_at
            );
            users.push(user);
        }

        return Ok(AdminDashboardTemplate { applicants: users });
    }
    Ok(AdminDashboardTemplate {
        applicants: Vec::new(),
    })
}
