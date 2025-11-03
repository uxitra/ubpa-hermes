use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "privacy.html")]
pub struct PrivacyTemplate;
