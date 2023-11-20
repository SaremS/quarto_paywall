use std::path::PathBuf;

use actix_files as fs;
use actix_session::Session;
use actix_web::{
    web::{Bytes, Data},
    HttpRequest, HttpResponse, Responder, Result,
};
use futures::stream::once;

use crate::inmemory_html_server::InMemoryHtml;
use crate::inmemory_static_files::InMemoryStaticFiles;
use crate::security::session_status_from_session;

pub async fn index(
    req: HttpRequest,
    in_memory_html: Data<InMemoryHtml>,
    session: Session,
) -> Result<impl Responder> {
    let session_status = session_status_from_session(&session, &req).await;

    let output = in_memory_html
        .get("index.html", &session_status)
        .await;

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

pub async fn in_memory_static_files(req: HttpRequest, in_memory_static: Data<InMemoryStaticFiles>) -> Result<impl Responder> {
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

pub async fn html_files(
    req: HttpRequest,
    in_memory_html: Data<InMemoryHtml>,
    session: Session,
) -> Result<impl Responder> {
    let session_status = session_status_from_session(&session, &req).await;
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
}