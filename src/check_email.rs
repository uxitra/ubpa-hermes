pub fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();

    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    if local.is_empty() || domain.is_empty() {
        return false;
    }

    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    if email.contains(' ') {
        return false;
    }

    true
}
