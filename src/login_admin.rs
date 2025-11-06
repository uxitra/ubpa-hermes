use crate::templates::admin_template::AdminTemaplate;

/// Returns the admin login site
pub async fn login_admin() -> Result<impl actix_web::Responder, actix_web::Error> {
    Ok(AdminTemaplate {})
}
