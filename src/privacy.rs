use crate::templates::privacy_template::PrivacyTemplate;
use actix_web::Error;
use actix_web::Responder;

pub async fn privacy() -> Result<impl Responder, Error> {
    Ok(PrivacyTemplate {})
}
