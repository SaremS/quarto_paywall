use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref VALID_ROLE: Regex = Regex::new(r"^(admin|user)$").unwrap();
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct RegisterUser {
    #[validate(email(message = "Invalid email format."))]
    pub email: String,
    #[validate(length(min = 1, message = "Username cannot be empty."))]
    pub username: String,
    #[validate(must_match(other = "password_repeat", message = "Passwords must match."))]
    pub password: String,
    pub password_repeat: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UserCreated {
    pub username: String,
    pub jwt: String,
}

#[derive(Debug, Validate, Serialize, Clone)]
pub struct User {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub username: String,
    pub password: String,
    pub is_verified: bool,
    #[validate(regex = "VALID_ROLE")]
    pub role: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLoggedIn {
    pub username: String,
    pub jwt: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    #[test]
    fn register_user_valid() {
        let register_user = super::RegisterUser {
            email: "asdf@asdf.com".to_string(),
            username: "test".to_string(),
            password: "ASDFASDF".to_string(),
            password_repeat: "ASDFASDF".to_string(),
        };

        assert_eq!(register_user.validate(), Ok(()));
    }

    #[test]
    fn register_user_invalid_email() {
        let register_user = super::RegisterUser {
            email: "asdfasdf.com".to_string(),
            username: "test".to_string(),
            password: "ASDFASDF".to_string(),
            password_repeat: "ASDFASDF".to_string(),
        };

        let got_error = match register_user.validate() {
            Ok(()) => false,
            Err(_) => true,
        };

        assert!(got_error);
    }

    #[test]
    fn register_user_nonmatching_passwords() {
        let register_user = super::RegisterUser {
            email: "asdf@asdf.com".to_string(),
            username: "test".to_string(),
            password: "ASDFASDFa".to_string(),
            password_repeat: "ASDFASDF".to_string(),
        };

        let got_error = match register_user.validate() {
            Ok(()) => false,
            Err(_) => true,
        };

        assert!(got_error);
    }
}
