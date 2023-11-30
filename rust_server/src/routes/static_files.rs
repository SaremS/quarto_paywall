use std::{path::PathBuf, collections::HashMap};

use actix_files as fs;
use actix_session::Session;
use actix_web::{
    web::{Bytes, Data},
    HttpRequest, HttpResponse, Responder, Result,
};
use futures::stream::once;

use crate::database::Database;
use crate::envvars::EnvVarLoader;
use crate::inmemory_html_server::InMemoryHtml;
use crate::inmemory_static_files::InMemoryStaticFiles;
use crate::models::{AuthLevel, SessionStatus};
use crate::security::session_status_from_session;
use crate::paywall::{PaywallItem, AuthLevelConditionalObject, PaywallServer};


pub async fn index(
    in_memory_html: Data<InMemoryHtml>,
    session: Session,
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let session_status =
        session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;

    let output = in_memory_html.get("index.html", &session_status).await;

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


pub async fn in_memory_static_files(
    req: HttpRequest,
    in_memory_static: Data<InMemoryStaticFiles>,
) -> Result<impl Responder> {
    let path: String = req.match_info().query("filename").parse().unwrap();
    let output = in_memory_static.get(&path).await;

    match output {
        Some(html) => {
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html))
        }
        None => return Ok(HttpResponse::NotFound().body("Not found")),
    }
}

type HashMapServer = HashMap<String, PaywallItem<String, AuthLevelConditionalObject<String>>>;

pub async fn html_files(
    req: HttpRequest,
    hash_map_server: Data<HashMapServer>,
    db: Data<dyn Database>,
    session: Session,
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let mut session_status =
        session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;

    inplace_update_auth(&mut session_status, db, &req).await;

    let query_path: String = req.match_info().query("filename").parse().unwrap();
    let path = format!("{}/{}", env_var_loader.get_path_static_files(), &query_path); 

    let output = hash_map_server.get_content(&path, &session_status).await;

    match output {
        Some(html) => {
            let body = once(async move { Ok::<_, actix_web::Error>(Bytes::from(html)) });
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .streaming(body));
        }
        None => return Ok(HttpResponse::NotFound().body("Not found")),
    }
}

/*pub async fn html_files(
    req: HttpRequest,
    in_memory_html: Data<InMemoryHtml>,
    db: Data<dyn Database>,
    session: Session,
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let mut session_status =
        session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;

    inplace_update_auth(&mut session_status, db, &req).await;

    let path: String = req.match_info().query("filename").parse().unwrap();

    let output = in_memory_html.get(&path, &session_status).await;

    match output {
        Some(html) => {
            let body = once(async move { Ok::<_, actix_web::Error>(Bytes::from(html)) });
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .streaming(body));
        }
        None => return Ok(HttpResponse::NotFound().body("Not found")),
    }
}*/

async fn inplace_update_auth(
    session_status: &mut SessionStatus,
    db: Data<dyn Database>,
    http_request: &HttpRequest,
) {
    if session_status.auth_level == AuthLevel::UserUnconfirmed
    {
        let target_article = http_request.match_info().as_str();
        let user_id = session_status.user_id.unwrap();

        if db
            .user_id_has_access_by_link(user_id, &target_article.to_string())
            .await
        {
            session_status.auth_level = AuthLevel::PaidAuth;
        } else if db.user_id_is_verified(user_id).await {
            session_status.auth_level = AuthLevel::UserConfirmed;
        }
    }
}
