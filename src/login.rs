use crate::templates::login_template::LoginTemaplate;

/// Returns the login site
pub async fn login() -> Result<impl actix_web::Responder, actix_web::Error> {
    Ok(LoginTemaplate {})
}
