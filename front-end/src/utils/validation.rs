pub fn validate_password(password: String) -> bool {
    let has_min_length = password.len() >= 8;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special_char = password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:'\",.<>?/`~".contains(c));

    has_min_length && has_uppercase && has_lowercase && has_digit && has_special_char
}
