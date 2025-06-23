use regex::Regex;

use crate::models::errors::{AppError, AppResult};

pub fn validate_username(username: &str) -> AppResult<()> {
    // length: 4 < x < 16
    // chars: a-Z, 0-9

    if username.len() < 4 || username.len() > 16 {
        return Err(AppError::BadUsernameLength);
    }

    let alphanumeric = username.chars().all(|c| c.is_alphanumeric());
    if !alphanumeric {
        return Err(AppError::InvalidUsername);
    }

    Ok(())
}

pub fn validate_email(email: &str) -> AppResult<()> {
    // check if email is empty or doesn't contain '@'
    if email.is_empty() || !email.contains('@') {
        return Err(AppError::InvalidEmailFormat);
    }

    // split domain and user part
    let parts = email.split('@').collect::<Vec<_>>();
    let (user_part, domain_part) = (parts[0], parts[1]);

    // validate length of each part
    if user_part.len() > 64 || domain_part.len() > 255 {
        return Err(AppError::EmailTooLong);
    }

    // check user part w/ regex
    let user_regex = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap();
    if !user_regex.is_match(user_part) {
        return Err(AppError::InvalidEmailFormat);
    }

    // validate domain part
    let domain_regex = Regex::new(r"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    if !domain_regex.is_match(domain_part) {
        return Err(AppError::InvalidEmailFormat);
    }

    Ok(())
}

// TODO: check if password contains at least one uppercase and lowercase letter
pub fn validate_password(password: &str) -> AppResult<()> {
    // length: 8 < x < 2048
    // contains: 2 symbols, 3 digits

    if password.len() < 8 || password.len() > 2048 {
        return Err(AppError::BadPasswordLength);
    }

    let symbols = password
        .chars()
        .filter(|c| {
            c.is_ascii_punctuation() ||
            "!@#$%^&*()-_=+[]{}|;:'\",.<>?/\\`~".contains(*c)
        })
        .count();
    let digits = password.chars().filter(|c| c.is_ascii_digit()).count();

    if symbols < 2 {
        return Err(AppError::PasswordNotEnoughSymbols);
    } else if digits < 3 {
        return Err(AppError::PasswordNotEnoughDigits);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{validate_email, validate_password, validate_username};

    #[test]
    fn test_valid_usernames() {
        let usernames = [
            "abcd",
            "aaaaaaaaaaaaaaaa",
            "nest12364",
            "1827392732",
        ];

        for username in usernames {
            let valid = validate_username(username).is_ok();
            assert!(valid, "{username}");
        }
    }

    #[test]
    fn test_invalid_usernames() {
        let usernames = [
            "abc",
            "aaaaaaaaaaaaaaaaa",
            "a-h-a",
            "a_h_a",
            "wl adyslaw",
            "special$",
            "!@#$%^&*()",
        ];

        for username in usernames {
            let valid = validate_username(username).is_err();
            assert!(valid, "{username}");
        }
    }

    #[test]
    fn test_valid_email() {
        let emails = [
            "user@example.com",
            "with.sub.domain@extra.com",
            // r#""very.unusual.@.unusual.com"@example.com"#,
            // r#""much.more unusual\"@example.com"#,
            // r#""very.(),:;<>[]\".VERY.\"very@\\ \"very\".unusual"@example.com"#,
            "admin@mailserver1",
            // "user@[192.168.0.1]",
            // "user@[IPv6:2001:db8::1]",
            "user+tag@example.com",
            "customer/department=shipping@example.com",
            "!def!xyz%abc@example.com",
            "_Yosemite.Sam@example.com",
            "~@example.com",
            // "あいうえお@example.com",
            // r#""john..doe"@example.com"#,
            // r#"" "@example.org"#,
        ];

        for email in emails {
            let valid = validate_email(email).is_ok();
            assert!(valid, "{email}");
        }
    }

    #[test]
    fn test_invalid_email() {
        let emails = [
            "plainaddress",
            "@no-local-part.com",
            "Outlook Contact <outlook-contact@domain.com>",
            "no-at.domain.com",
            "user@.invalid.com",
            "user@invalid..com",
            ".user@example.com",
            "user.@example.com",
            "user..name@example.com",
            "user@example..com",
            "user@-example.com",
            "user@example.com.",
            "user@.com",
            "user@com",
            "user@exam_ple.com",
        ];

        for email in emails {
            let valid = validate_email(email).is_err();
            assert!(valid, "{email}");
        }
    }

    #[test]
    fn test_valid_password() {
        let values = [
            "abc!e123",
            "@ea532@#@!!!",
            "3aha2aha1aha@",
        ];

        for value in values {
            let valid = validate_password(value).is_ok();
            assert!(valid, "{value}");
        }
    }
}
