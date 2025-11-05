use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "dashboard.html")]
/// Askama template struct representing the upload.html file
pub struct AdminDashboardTemplate {
    #[allow(dead_code)]
    pub applicants: Vec<String>,
}
