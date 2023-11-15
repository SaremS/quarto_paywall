use std::path::PathBuf;

use actix_files as fs;
use actix_session::Session;
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder, Result,
};
use askama::Template;

use crate::database::Database;
use crate::inmemory_html_server::InMemoryHtml;
use crate::models::{LoginUser, RegisterUser, Role};
use crate::security::{authorize_with_cookie, AuthLevel, SessionStatus};
use crate::templates::{
    LoginSuccessTemplate, LoginTemplate, LogoutSuccessTemplate, RegisterSuccessTemplate,
    RegisterTemplate, UberTemplate, UserDashboardTemplate,
};

pub async fn index(in_memory_html: Data<InMemoryHtml>, session_status: SessionStatus) -> Result<impl Responder> {
    let output = in_memory_html.get("index.html".to_string(), session_status).await;

    match output {
        Some(html) => {
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html))
        }
        None => return Ok(HttpResponse::NotFound().body("Not found")),
    }
}

pub async fn static_files(req: HttpRequest) -> Result<fs::NamedFile> {
    let mut path = PathBuf::new();
    path.push("../paywall_blog/_site/");

    let path_suffix: PathBuf = req.match_info().query("filename").parse().unwrap();
    path.push(path_suffix);
    Ok(fs::NamedFile::open(path)?)
}

pub async fn html_files(
    req: HttpRequest,
    in_memory_html: Data<InMemoryHtml>,
    session_status: SessionStatus,
) -> Result<impl Responder> {
    let path: String = req.match_info().query("filename").parse().unwrap();

    let output = in_memory_html.get(path, session_status).await;

    match output {
        Some(html) => {
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html))
        }
        None => return Ok(HttpResponse::NotFound().body("Not found")),
    }
}

pub async fn get_user_dashboard(session_status: SessionStatus) -> Result<impl Responder> {
    let auth_level = session_status.auth_level;

    let content;
    if auth_level > AuthLevel::NoAuth {
        content = UserDashboardTemplate {
            username: session_status.username.unwrap(),
        }
        .render()
        .unwrap();
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

pub async fn get_user_dashboard_template(session_status: SessionStatus) -> Result<impl Responder> {
    let content;

    if session_status.auth_level > AuthLevel::NoAuth {
        content = UserDashboardTemplate { username: session_status.username.unwrap() }.render().unwrap();
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

pub async fn get_paywalled_content(req: HttpRequest, session_status: SessionStatus) -> Result<fs::NamedFile> {
    let mut path;
    let filename: PathBuf = req.match_info().query("filename").parse().unwrap();

    if session_status.auth_level > AuthLevel::NoAuth {
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
    let login_template = LogoutSuccessTemplate {}.render().unwrap();

    let _ = session.remove("session");

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(login_template);

    return Ok(response);
}
