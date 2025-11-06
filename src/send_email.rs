use lettre::Transport;

#[derive(serde::Deserialize, Clone)]
pub struct EmailConfig {
    pub email: String,
    pub password: String,
    pub subject: String,
    pub email_content: String,
}

/// Sends an email to a pre configured email adress
pub fn send_email(email: String, config: &EmailConfig) {
    // Create the email
    let lettre_email = lettre::Message::builder()
        .from(config.email.parse().unwrap())
        .to(email.parse().unwrap())
        .subject(&config.subject)
        .body(config.email_content.clone())
        .unwrap();

    let creds = lettre::transport::smtp::authentication::Credentials::new(
        config.email.clone(),
        config.password.clone(),
    );

    // Create email mailer
    let mailer = lettre::SmtpTransport::relay("smtp.web.de")
        .unwrap()
        .credentials(creds)
        .build();

    // Send email to given email adress
    match mailer.send(&lettre_email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {}", e),
    }
}
