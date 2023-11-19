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

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum AuthLevel {
    /*
    Assesses the different levels of authentication to read a page;
    values are in ascending order of permissions - only confirmed users
    can become paid users. Admin has highest permissions but can always be assigned
    */
    NoAuth = 1,          //if user is not logged in
    UserUnconfirmed = 2, //if user is registered and logged in, but without email confirm yet
    UserConfirmed = 3,   //logged in and confirmed via email
    PaidAuth = 4,        //if user has paid for article
    AdminAuth = 5,       //highest level, access to everything - for future reference
}

impl AuthLevel {
    fn as_u8(&self) -> u8 {
        return *self as u8;
    }
}

impl PartialEq for AuthLevel {
    fn eq(&self, other: &Self) -> bool {
        return self.as_u8() == other.as_u8();
    }
}

impl PartialOrd for AuthLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.as_u8().partial_cmp(&other.as_u8());
    }
}

pub struct SessionStatus {
    pub user_id: Option<usize>,
    pub auth_level: AuthLevel,
    pub username: Option<String>,
}
