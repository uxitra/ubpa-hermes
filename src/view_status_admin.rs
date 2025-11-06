use sqlx::SqlitePool;

use crate::dashboard;
use crate::templates::admin_dashboard::AdminDashboardTemplate;

#[derive(serde::Deserialize, Debug)]
pub struct AdminLoginForm {
    pub password: String,
    pub username: String,
}

/// Returns the admin dashboard if a password was suplied
pub async fn view_status_admin(
    form: actix_web::web::Form<AdminLoginForm>,
    pool: actix_web::web::Data<SqlitePool>,
) -> Result<impl actix_web::Responder, actix_web::Error> {
    /*let username =
        std::env::var("ADMIN_USERNAME").expect("Failed to get enviroment variable 'ADMIN_USERNAME' ");
    let password =
        std::env::var("ADMIN_PASSWORD").expect("Failed to get enviroment variable 'ADMIN_PASSWORD' ");*/

    if form.username == "admin" && form.password == "12345" {
        let pool = pool.get_ref();

        return dashboard::dashboard(pool).await;
    }
    Ok(AdminDashboardTemplate {
        applicants: Vec::new(),
    })
}
