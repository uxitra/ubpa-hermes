/// Check if a given string has a valid email format
pub fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();

    // A normal email adress contains one '@' so it should have two parts after spliting at the '@'
    if parts.len() != 2 {
        return false;
    }

    // Get both parts of an email adress
    let local = parts[0];
    let domain = parts[1];

    // If one of them is empty the email adress hasnt a valid format
    if local.is_empty() || domain.is_empty() {
        return false;
    }

    // If the domain part of the email contains a '.' at the end or the start it isnt valid
    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    // The email also isnt valid if it contains a space
    if email.contains(' ') {
        return false;
    }

    true
}
