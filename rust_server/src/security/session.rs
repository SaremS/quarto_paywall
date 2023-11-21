use actix_session::{
    config::{BrowserSession, CookieContentSecurity},
    storage::CookieSessionStore,
    Session, SessionMiddleware,
};
use actix_web::cookie::{Key, SameSite};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::models::{AuthLevel, Claims, Role, SessionStatus, User};

pub fn make_session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
        .cookie_name(String::from("session"))
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_http_only(true)
        .build()
}

pub async fn session_status_from_session(session: &Session, jwt_secret_key: &str) -> SessionStatus {
    let cookie_result = session.get::<String>("session");
    let cookie_auth_specs = match cookie_result {
        Ok(cookie_option) => extract_cookie_auth_specs(cookie_option, jwt_secret_key).await,
        Err(_) => CookieAuthSpecs {
            has_access: false,
            is_admin: false,
            username: None,
            user_id: None,
        },
    };

    let CookieAuthSpecs {
        has_access,
        is_admin,
        username,
        user_id,
    } = cookie_auth_specs;

    let session_status = if has_access && is_admin {
        SessionStatus {
            user_id: Some(user_id.unwrap()),
            auth_level: AuthLevel::AdminAuth,
            username: Some(username.unwrap()),
        }
    } else if has_access {
        SessionStatus {
            user_id: Some(user_id.unwrap()),
            auth_level: AuthLevel::UserUnconfirmed,
            username: Some(username.unwrap()),
        }
    } else {
        SessionStatus {
            user_id: None,
            auth_level: AuthLevel::NoAuth,
            username: None,
        }
    };

    return session_status;
}

pub fn get_jwt_for_user(user: &User, jwt_secret_key: &str) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("invalid timestamp")
        .timestamp();

    let user_claims = Claims {
        sub: user.username.clone(),
        role: user.role.clone(),
        user_id: user.id,
        exp: expiration_time as usize,
    };

    let token = match encode(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_secret(jwt_secret_key.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    return token;
}

fn is_admin(claims_role: &str) -> bool {
    let claims_role = Role::from_str(claims_role);

    return claims_role == Role::Admin;
}

async fn extract_cookie_auth_specs(token_option: Option<String>, jwt_secret_key: &str) -> CookieAuthSpecs {
    let token;
    match token_option {
        Some(t) => token = t,
        None => {
            return CookieAuthSpecs {
                has_access: false,
                is_admin: false,
                username: None,
                user_id: None,
            }
        }
    };
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret_key.as_bytes()),
        &Validation::default(),
    );
    match decoded {
        Ok(d) => {
            if is_admin(&d.claims.role) {
                return CookieAuthSpecs {
                    has_access: true,
                    is_admin: true,
                    username: Some(d.claims.sub),
                    user_id: Some(d.claims.user_id),
                };
            } else {
                return CookieAuthSpecs {
                    has_access: true,
                    is_admin: false,
                    username: Some(d.claims.sub),
                    user_id: Some(d.claims.user_id),
                };
            }
        }
        Err(_) => {
            return CookieAuthSpecs {
                has_access: false,
                is_admin: false,
                username: None,
                user_id: None,
            };
        }
    }
}

//helper return struct to reduce the risk of accidentally confusing the bools involved
struct CookieAuthSpecs {
    has_access: bool,
    is_admin: bool,
    username: Option<String>,
    user_id: Option<usize>,
}
