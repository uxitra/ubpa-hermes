use std::collections::HashMap;

use sqlx::Row;
use sqlx::SqlitePool;

use crate::state::State;
use crate::templates::admin_dashboard::AdminDashboardTemplate;

pub async fn dashboard(pool: &SqlitePool) -> Result<AdminDashboardTemplate, actix_web::Error> {
    // Create hashmap for handling the users
    let mut users = HashMap::new();

    // Get all rows
    let rows = sqlx::query("SELECT key, value, state, created_at FROM applicants")
        .fetch_all(pool)
        .await
        .unwrap();

    // iterate over the rows and insert the values into the map
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
        users.insert(key, user);
    }

    // return an admin dashboard template with a suplied vector created from the map
    Ok(AdminDashboardTemplate {
        applicants: users.into_iter().collect::<Vec<_>>(),
    })
}
