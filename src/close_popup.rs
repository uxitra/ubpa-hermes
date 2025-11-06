use actix_web::Error;
use actix_web::Responder;

/// Close the popup by clearing the div
pub async fn close_modal() -> Result<impl Responder, Error> {
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("")) // clears the target
}
