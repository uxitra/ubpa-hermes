use crate::check_email::is_valid_email;
use crate::check_pdf::detect_pdf;
use crate::templates::upload_template::UploadTemplate;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::bytes::Bytes;
use actix_multipart::form::text::Text;
use actix_web::Error;
use actix_web::Responder;

#[derive(Debug, MultipartForm)]
/// Contains all the variables that are send to the backend from the html form
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<Bytes>,
    #[multipart(rename = "email")]
    email: Option<Text<String>>,
    #[multipart(rename = "checkbox")]
    checkbox: Option<Text<String>>,
}

/// Gets the post request from the html form with all atributes
pub async fn load(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder, Error> {
    if form.files.is_empty() {
        println!("You should send files");
        return Ok(UploadTemplate {
            error: "You must upload a file",
        });
    }

    let count = form.files.len();

    if count > 10 {
        return Ok(UploadTemplate {
            error: "To many files (max: 10)",
        });
    }

    for f in form.files {
        let filename = f.file_name.unwrap();

        println!("working with file: {}", filename);

        if f.data.len() > 10 * 1024 * 1024 {
            println!("to big");
            return Ok(UploadTemplate {
                error: "file is to big",
            });
        }

        let path = format!("./upload/{}", filename);

        if !detect_pdf(&f.data) {
            return Ok(UploadTemplate {
                error: "file must be a pdf",
            });
        } else {
            // add path into the database
            std::fs::write(path, f.data).expect("failed to write data");
            println!("file uploaded");
        }
    }

    match form.email {
        Some(email) => {
            if email.is_empty() {
                return Ok(UploadTemplate {
                    error: "no email suplied",
                });
            } else if is_valid_email(&email) {
                println!("{}", email.as_str())
            } else {
                return Ok(UploadTemplate {
                    error: "email must be a valid format",
                });
            }
        }
        None => {
            return Ok(UploadTemplate {
                error: "no email suplied",
            });
        }
    }

    let checkbox_value = form.checkbox.as_ref().map(|t| t.0 == "on").unwrap_or(false);

    if !checkbox_value {
        return Ok(UploadTemplate {
            error: "you need to agree to the Nutzerbedingungen",
        });
    }

    Ok(UploadTemplate { error: "" })
}
