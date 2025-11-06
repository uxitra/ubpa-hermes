use lettre::Transport;

#[derive(serde::Deserialize)]
struct EmailConfig {
    email: String,
    password: String,
    subject: String,
    email_content: String,
}

/// Sends an email to a pre configured email adress
pub fn send_email(email: String) {
    let config_data = std::fs::read_to_string("./config.json")
        .expect("Failed to read config.json please check if the file was suplied (the progrma doesnt automatically create it) check ");
    let config: EmailConfig = serde_json::from_str(&config_data).expect("Failed to parse JSON");

    // Create the email
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
