use crate::templates::login_template::LoginTemaplate;

pub async fn login() -> Result<impl actix_web::Responder, actix_web::Error> {
    Ok(LoginTemaplate {})
}
