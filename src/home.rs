use crate::templates::home_template::HomeTemplate;
use actix_web::{Error, Responder};

/// Load the homepage of the site
pub async fn home() -> Result<impl Responder, Error> {
    println!("loaded home!");
    Ok(HomeTemplate {})
}
