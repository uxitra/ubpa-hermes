use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "privacy.html")]
/// Askama template struct representing the upload.html file
pub struct PrivacyTemplate;
