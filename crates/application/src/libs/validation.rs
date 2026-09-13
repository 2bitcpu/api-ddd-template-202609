pub fn is_valid_password(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    if !s.chars().all(|c| {
        c.is_ascii_alphanumeric()
            || matches!(
                c,
                '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '-' | '_' | ':' | '.' | '/'
            )
    }) {
        return false;
    }

    let has_upper = s.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = s.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = s.chars().any(|c| c.is_ascii_digit());
    let has_symbol = s.chars().any(|c| {
        matches!(
            c,
            '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '-' | '_' | ':' | '.' | '/'
        )
    });
    has_upper && has_lower && has_digit && has_symbol && s.len() >= 8 && s.len() <= 64
}

pub fn is_valid_account(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '@'))
        && s.len() >= 6
        && s.len() <= 32
}
