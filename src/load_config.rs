use crate::send_email::EmailConfig;

pub async fn load_config() -> EmailConfig {
    let config_data = std::fs::read_to_string("./config.json")
        .expect("Failed to read config.json please check if the file was suplied (the progrma doesnt automatically create it) check ");
    let config: EmailConfig = serde_json::from_str(&config_data).expect("Failed to parse JSON");
    config
}
