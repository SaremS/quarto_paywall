use std::path::PathBuf;

use actix_files as fs;
use actix_session::Session;
use actix_web::{
    http,
    web::{Bytes, Data},
    HttpRequest, HttpResponse, Responder, Result,
};
use futures::stream::once;

use crate::database::Database;
use crate::envvars::EnvVarLoader;
use crate::models::{AuthLevel, SessionStatus};
use crate::paywall::{
    AuthLevelConditionalObject, ContentAndHash, FileserverWrapper, OptionOrHashMatch, PaywallServer,
};
use crate::security::session_status_from_session;

pub async fn index<V: PaywallServer<String, AuthLevelConditionalObject<String>> + std::marker::Sync>(
    req: HttpRequest,
    server: Data<V>,
    session: Session,
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let session_status =
        session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;

    let path = "/index.html";

    let content_or_etag_match = server
        .get_content_if_different_etag(
            &path,
            &session_status,
            req.headers().get(http::header::IF_NONE_MATCH),
        )
        .await;

    match content_or_etag_match {
        OptionOrHashMatch::Some(ContentAndHash { content, hash }) => {
            let body = once(async move { Ok::<_, actix_web::Error>(Bytes::from(content)) });
            return Ok(HttpResponse::Ok()
                .insert_header((http::header::ETAG, hash))
                .content_type("text/html; charset=utf-8")
                .streaming(body));
        }
        OptionOrHashMatch::HashMatch => {
            return Ok(HttpResponse::NotModified().finish());
        }
        OptionOrHashMatch::None => {
            return Ok(HttpResponse::NotFound().body("Not found"));
        }
    }
}

pub async fn static_files(req: HttpRequest) -> Result<fs::NamedFile> {
    let mut path = PathBuf::new();
    path.push("../paywall_blog/_site/");

    let path_suffix: PathBuf = req.match_info().query("filename").parse().unwrap();
    path.push(path_suffix);
    Ok(fs::NamedFile::open(path)?)
}

pub async fn in_memory_static_files<
    V: PaywallServer<String, AuthLevelConditionalObject<String>> + std::marker::Sync,
>(
    req: HttpRequest,
    session: Session,
    server: Data<FileserverWrapper<V>>,
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let session_status =
        session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;

    let query_path: String = req.match_info().query("filename").parse().unwrap();
    let path = format!("/{}", query_path);

    let content_or_etag_match = server
        .server
        .get_content_if_different_etag(
            &path,
            &session_status,
            req.headers().get(http::header::IF_NONE_MATCH),
        )
        .await;

    match content_or_etag_match {
        OptionOrHashMatch::Some(ContentAndHash { content, hash }) => {
            let body = once(async move { Ok::<_, actix_web::Error>(Bytes::from(content)) });
            return Ok(HttpResponse::Ok()
                .insert_header((http::header::ETAG, hash))
                .content_type("text/html; charset=utf-8")
                .streaming(body));
        }
        OptionOrHashMatch::HashMatch => {
            return Ok(HttpResponse::NotModified().finish());
        }
        OptionOrHashMatch::None => {
            return Ok(HttpResponse::NotFound().body("Not found"));
        }
    }
}

pub async fn html_files<
    V: PaywallServer<String, AuthLevelConditionalObject<String>> + std::marker::Sync,
>(
    req: HttpRequest,
    server: Data<V>,
    db: Data<dyn Database>,
    session: Session,
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let mut session_status =
        session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;

    inplace_update_auth(&mut session_status, db, &req).await;

    let query_path: String = req.match_info().query("filename").parse().unwrap();
    let path = format!("/{}", query_path);

    let content_or_etag_match = server
        .get_content_if_different_etag(
            &path,
            &session_status,
            req.headers().get(http::header::IF_NONE_MATCH),
        )
        .await;

    match content_or_etag_match {
        OptionOrHashMatch::Some(ContentAndHash { content, hash }) => {
            let body = once(async move { Ok::<_, actix_web::Error>(Bytes::from(content)) });
            return Ok(HttpResponse::Ok()
                .insert_header((http::header::ETAG, hash))
                .content_type("text/html; charset=utf-8")
                .streaming(body));
        }
        OptionOrHashMatch::HashMatch => {
            return Ok(HttpResponse::NotModified().finish());
        }
        OptionOrHashMatch::None => {
            return Ok(HttpResponse::NotFound().body("Not found"));
        }
    }
}

async fn inplace_update_auth(
    session_status: &mut SessionStatus,
    db: Data<dyn Database>,
    http_request: &HttpRequest,
) {
    if session_status.auth_level == AuthLevel::UserUnconfirmed {
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
