use log::debug;
use std::fmt;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};

use crate::models::{Claims, User};

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

pub fn get_hashed_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Scrypt
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    return password_hash;
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash).unwrap();

    return Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
}

fn get_secret() -> Vec<u8> {
    return std::env::var("JWT_SECRET").unwrap().into_bytes();
}

pub fn get_jwt_for_user(user: User) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("invalid timestamp")
        .timestamp();
    let user_claims = Claims {
        sub: user.username,
        role: user.role,
        exp: expiration_time as usize,
    };

    let token = match encode(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_secret(&get_secret()),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    return token;
}

fn is_authorized(required_role: Role, claims_role: &str) -> bool {
    let claims_role = Role::from_str(claims_role);
    debug!("needed role: {}, user role: {}", required_role, claims_role);

    return required_role == claims_role || claims_role == Role::Admin;
}

pub async fn authorize_with_cookie((role, token_option): (Role, Option<String>)) -> (bool, String) {
    let token;

    match token_option {
        Some(t) => token = t,
        None => return (false, "".to_string()),
    }

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&get_secret()),
        &Validation::default(),
    );

    match decoded {
        Ok(d) => {
            debug!("decoded claims: {:?}", d.claims);
            if is_authorized(role, &d.claims.role) {
                return (true, d.claims.sub);
            } else {
                return (false, "".to_string());
            }
        }
        Err(_) => return (false, "".to_string()),
    }
}
