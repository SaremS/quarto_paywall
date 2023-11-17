use std::fmt;
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Clone, PartialEq, Debug)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role.to_lowercase().as_str() {
            "admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
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
    pub user_id: usize,
    pub accessible_articles: Vec<String>,
    pub exp: usize,
}
