use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "upload.html")]
/// Displays an given error on a html div
pub struct UploadTemplate<'a> {
    pub error: &'a str,
    pub token: String,
}
