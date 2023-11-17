use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_session::{
    config::{BrowserSession, CookieContentSecurity},
    storage::CookieSessionStore,
    Session, SessionExt, SessionMiddleware,
};
use actix_web::{
    cookie::{Key, SameSite},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, FromRequest, HttpMessage, HttpRequest,
};
use chrono::{Duration, Utc};
use futures::future::ok;
use futures_util::future::{FutureExt, LocalBoxFuture};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::models::{Claims, Role, User};
use crate::security::xor_hash;
use crate::utils::last_five_chars;

pub fn make_session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
        .cookie_name(String::from("session"))
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_http_only(true)
        .build()
}

pub struct AuthCheck {}

impl AuthCheck {
    pub fn new() -> Self {
        AuthCheck {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthCheckMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthCheckMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthCheckMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthCheckMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        async move {
            let target_element = req.match_info().as_str().split("/").last().unwrap();

            if !target_element.contains(".html")
                && !target_element.contains("user-dashboard")
                && target_element != "checkout"
            {
                let session_status = SessionStatus {
                    user_id: None,
                    auth_level: AuthLevel::NoAuth,
                    username: None,
                };
                req.extensions_mut().insert::<SessionStatus>(session_status);

                let res = srv.call(req).await?;
                return Ok(res);
            }

            let session = req.get_session();

            let target_hash = last_five_chars(&xor_hash(&target_element));

            let cookie_result = session.get::<String>("session");



        let cookie_auth_specs = match cookie_result {
            Ok(cookie_option) => extract_cookie_auth_specs(cookie_option, target_hash).await,
            Err(_) => CookieAuthSpecs {
                has_access: false,
                has_paid: false,
                is_admin: false,
                username: None,
                user_id: None,
            },
        };

        let CookieAuthSpecs {
            has_access,
            has_paid,
            is_admin,
            username,
            user_id,
        } = cookie_auth_specs;

            let auth_status = if has_access && is_admin {
                SessionStatus {
                    user_id: Some(user_id.unwrap()),
                    auth_level: AuthLevel::AdminAuth,
                    username: Some(username.unwrap()),
                }
            } else if has_access && has_paid {
                SessionStatus {
                    user_id: Some(user_id.unwrap()),
                    auth_level: AuthLevel::PaidAuth,
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

            req.extensions_mut().insert::<SessionStatus>(auth_status);
            let res = srv.call(req).await?;

            return Ok(res);
        }
        .boxed_local()
    }
}

//helper return struct to reduce the risk of accidentally confusing the bools involved
struct CookieAuthSpecs {
    has_access: bool,
    has_paid: bool,
    is_admin: bool,
    username: Option<String>,
    user_id: Option<usize>,
}

pub async fn session_status_from_session(
    session_option: Option<Session>,
    target_element: &str,
) -> SessionStatus {
    if let Some(session) = session_option {
        let target_hash = last_five_chars(&xor_hash(&target_element));
        let cookie_result = session.get::<String>("session");
        let cookie_auth_specs = match cookie_result {
            Ok(cookie_option) => extract_cookie_auth_specs(cookie_option, target_hash).await,
            Err(_) => CookieAuthSpecs {
                has_access: false,
                has_paid: false,
                is_admin: false,
                username: None,
                user_id: None,
            },
        };

        let CookieAuthSpecs {
            has_access,
            has_paid,
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
        } else if has_access && has_paid {
            SessionStatus {
                user_id: Some(user_id.unwrap()),
                auth_level: AuthLevel::PaidAuth,
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
    } else {
        let session_status = SessionStatus {
            user_id: None,
            auth_level: AuthLevel::NoAuth,
            username: None,
        };

        return session_status;
    }
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

impl FromRequest for SessionStatus {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let binding = req.extensions();
        let session_status = binding.get::<SessionStatus>().unwrap();
        let owned_status = SessionStatus {
            user_id: session_status.user_id.clone(),
            auth_level: session_status.auth_level.clone(),
            username: session_status.username.clone(),
        };
        return ok(owned_status);
    }
}

fn get_secret() -> Vec<u8> {
    return std::env::var("JWT_SECRET").unwrap().into_bytes();
}

pub fn get_jwt_for_user(user: User) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("invalid timestamp")
        .timestamp();

    let acc_hash = user
        .accessible_articles
        .into_iter()
        .map(|x| last_five_chars(&xor_hash(&x)))
        .collect();
    let user_claims = Claims {
        sub: user.username,
        role: user.role,
        user_id: user.id,
        accessible_articles: acc_hash,
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

fn is_admin(claims_role: &str) -> bool {
    let claims_role = Role::from_str(claims_role);

    return claims_role == Role::Admin;
}

async fn extract_cookie_auth_specs (
    token_option: Option<String>,
    target_hash: String,
) -> CookieAuthSpecs {
    let token;
    match token_option {
        Some(t) => token = t,
        None => {
            return CookieAuthSpecs {
                has_access: false,
                has_paid: false,
                is_admin: false,
                username: None,
                user_id: None,
            }
        }
    };
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&get_secret()),
        &Validation::default(),
    );
    match decoded {
        Ok(d) => {
            if is_admin(&d.claims.role) {
                return CookieAuthSpecs {
                    has_access: true,
                    has_paid: true,
                    is_admin: true,
                    username: Some(d.claims.sub),
                    user_id: Some(d.claims.user_id),
                };
            } else {
                return CookieAuthSpecs {
                    has_access: true,
                    has_paid: d.claims.accessible_articles.contains(&target_hash),
                    is_admin: false,
                    username: Some(d.claims.sub),
                    user_id: Some(d.claims.user_id),
                };
            }
        }
        Err(_) => {
            return CookieAuthSpecs {
                has_access: false,
                has_paid: false,
                is_admin: false,
                username: None,
                user_id: None,
            };
        }
    }
}
