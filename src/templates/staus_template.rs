use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "status.html")]
/// Askama template struct representing the upload.html file
pub struct StatusTemplate<'a> {
    pub error: &'a str,
}
