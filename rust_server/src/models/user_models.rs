use std::collections::HashSet; 

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
    pub user_id: usize,
    pub email: String,
    pub username: String,
    pub jwt: String,
}

#[derive(Debug, Validate, Serialize, Clone)]
pub struct User {
    pub id: usize,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub username: String,
    pub password: String,
    pub is_verified: bool,
    #[validate(regex = "VALID_ROLE")]
    pub role: String,
    pub accessible_articles: HashSet<String>,
}
