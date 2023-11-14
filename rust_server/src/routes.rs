use std::path::PathBuf;

use actix_files as fs;
use actix_session::Session;
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder, Result,
};
use askama::Template;

use crate::database::Database;
use crate::models::{LoginUser, RegisterUser};
use crate::security::{authorize_with_cookie, Role};
use crate::templates::{
    LoginTemplate, RegisterSuccessTemplate, RegisterTemplate, UberTemplate,
    UserDashboardTemplate, LoginSuccessTemplate, LogoutSuccessTemplate
};

pub async fn get_user_dashboard(session: Session) -> Result<impl Responder> {
    let cookie_result = session.get::<String>("session");

    let (has_auth, username) = match cookie_result {
        Ok(cookie_option) => authorize_with_cookie((Role::User, cookie_option)).await,
        Err(_) => (false, "".to_string()),
    };

    let content;

    if has_auth {
        content = UserDashboardTemplate { username }.render().unwrap();
    } else {
        content = LoginTemplate {
            error_message: "".to_string(),
        }
        .render()
        .unwrap();
    }

    let target_template = UberTemplate { content }.render().unwrap();

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(target_template);

    return Ok(response);
}

pub async fn get_user_dashboard_template(session: Session) -> Result<impl Responder> {
    let cookie_result = session.get::<String>("session");

    let (has_auth, username) = match cookie_result {
        Ok(cookie_option) => authorize_with_cookie((Role::User, cookie_option)).await,
        Err(_) => (false, "".to_string()),
    };

    let content;

    if has_auth {
        content = UserDashboardTemplate { username }.render().unwrap();
    } else {
        content = LoginTemplate {
            error_message: "".to_string(),
        }
        .render()
        .unwrap();
    }

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(content);

    return Ok(response);
}

pub async fn get_paywalled_content(req: HttpRequest, session: Session) -> Result<fs::NamedFile> {
    let cookie_result = session.get::<String>("session");

    let (has_auth, _) = match cookie_result {
        Ok(cookie_option) => authorize_with_cookie((Role::User, cookie_option)).await,
        Err(_) => (false, "".to_string()),
    };

    let mut path;
    let filename: PathBuf = req.match_info().query("filename").parse().unwrap();

    if has_auth {
        path = PathBuf::from("../paywall_blog/paywalled/");
        path.push(filename);
    } else {
        path = PathBuf::from("./templates/paywall.html");
    }

    return Ok(fs::NamedFile::open(path)?);
}

pub async fn get_register() -> Result<impl Responder> {
    let target_template = RegisterTemplate {
        error_message: "".to_string(),
    }
    .render()
    .unwrap();

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(target_template);

    return Ok(response);
}

pub async fn get_login() -> Result<impl Responder> {
    let target_template = LoginTemplate {
        error_message: "".to_string(),
    }
    .render()
    .unwrap();

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(target_template);

    return Ok(response);
}

pub async fn put_register_user(
    session: Session,
    user: Json<RegisterUser>,
    db: Data<dyn Database>,
) -> Result<impl Responder> {
    let create_user_result = db.create_user(user.into_inner()).await;

    let result_content;
    match create_user_result {
        Ok(user_created) => {
            result_content = RegisterSuccessTemplate {
                username: user_created.username,
            }
            .render()
            .unwrap();

            let _ = session.insert("session", user_created.jwt);
        }
        Err(e) => {
            result_content = RegisterTemplate {
                error_message: e.to_string(),
            }
            .render()
            .unwrap();
        }
    }

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(result_content);

    return Ok(response);
}

pub async fn put_login_user(
    session: Session,
    user: Json<LoginUser>,
    db: Data<dyn Database>,
) -> Result<impl Responder> {
    let login_user_result = db.login(user.into_inner()).await;

    let result_content;
    match login_user_result {
        Ok(user_logged_in) => {
            result_content = LoginSuccessTemplate {
                username: user_logged_in.username,
            }
            .render()
            .unwrap();

            let _ = session.insert("session", user_logged_in.jwt);
        }
        Err(e) => {
            result_content = LoginTemplate {
                error_message: e.to_string(),
            }
            .render()
            .unwrap();
        }
    }

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(result_content);

    return Ok(response);
}

pub async fn get_logout_user(session: Session) -> Result<impl Responder> {
    let login_template = LogoutSuccessTemplate {}
    .render()
    .unwrap();

    let _ = session.remove("session");

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(login_template);

    return Ok(response);
}
