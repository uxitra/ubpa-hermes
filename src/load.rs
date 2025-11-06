use crate::check_email::is_valid_email;
use crate::check_pdf::detect_pdf;
use crate::templates::upload_template::UploadTemplate;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::bytes::Bytes;
use actix_multipart::form::text::Text;
use actix_web::Error;
use actix_web::Responder;
use lettre::Transport;
use sqlx::SqlitePool;

#[derive(serde::Deserialize)]
struct EmailConfig {
    email: String,
    password: String,
    subject: String,
    email_content: String,
}

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
pub async fn load(
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: actix_web::web::Data<SqlitePool>,
) -> Result<impl Responder, Error> {
    // Check if they arent files sended
    if form.files.is_empty() {
        println!("You should send files");
        return Ok(UploadTemplate {
            error: "You must upload a file",
            token: "".to_string(),
        });
    }

    // Count the given files
    let count = form.files.len();

    if count > 10 {
        return Ok(UploadTemplate {
            error: "To many files (max: 10)",
            token: "".to_string(),
        });
    }

    // Iterate over the files and check if they are valid
    for f in form.files {
        let filename = f.file_name.unwrap();

        println!("working with file: {}", filename);

        if f.data.len() > 10 * 1024 * 1024 {
            println!("to big");
            return Ok(UploadTemplate {
                error: "file is to big",
                token: "".to_string(),
            });
        }

        let path = format!("./upload/{}", filename);

        if !detect_pdf(&f.data) {
            return Ok(UploadTemplate {
                error: "file must be a pdf",
                token: "".to_string(),
            });
        } else {
            // add path into the database
            std::fs::write(path, f.data).expect("failed to write data");
            println!("file uploaded");
        }
    }

    let email = form.email;

    // Check the email format
    match &email {
        Some(email) => {
            if email.is_empty() {
                return Ok(UploadTemplate {
                    error: "no email suplied",
                    token: "".to_string(),
                });
            } else if is_valid_email(email) {
                println!("{}", email.as_str())
            } else {
                return Ok(UploadTemplate {
                    error: "email must be a valid format",
                    token: "".to_string(),
                });
            }
        }
        None => {
            return Ok(UploadTemplate {
                error: "no email suplied",
                token: "".to_string(),
            });
        }
    }

    // Get the value of the checkbox
    let checkbox_value = form.checkbox.as_ref().map(|t| t.0 == "on").unwrap_or(false);

    if !checkbox_value {
        return Ok(UploadTemplate {
            error: "you need to agree to the Nutzerbedingungen",
            token: "".to_string(),
        });
    }

    let token = uuid::Uuid::new_v4();
    println!("{}", token);

    let email = email.unwrap().to_string();

    sqlx::query(r#"INSERT INTO users (key, value) VALUES (?, ?)"#)
        .bind(token.to_string())
        .bind(&email)
        .execute(pool.as_ref())
        .await
        .unwrap();
    println!("{}, {}", token, &email);

    let config_data = std::fs::read_to_string("./config.json")
        .expect("Failed to read config.json please check if the file was suplied");
    let config: EmailConfig = serde_json::from_str(&config_data).expect("Failed to parse JSON");

    let lettre_email = lettre::Message::builder()
        .from(config.email.parse().unwrap())
        .to(email.parse().unwrap())
        .subject(config.subject)
        .body(config.email_content)
        .unwrap();

    let creds = lettre::transport::smtp::authentication::Credentials::new(
        config.email.clone(),
        config.password.clone(),
    );

    let mailer = lettre::SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&lettre_email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {}", e),
    }

    // If succesfull return nothing
    Ok(UploadTemplate {
        error: "",
        token: token.to_string(),
    })
}
