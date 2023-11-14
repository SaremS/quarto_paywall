use actix_files as fs;
use actix_session::{storage::CookieSessionStore, SessionMiddleware, Session};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
};
use std::sync::Arc;

use actix_session::config::{BrowserSession, CookieContentSecurity};
use actix_web::{HttpRequest, HttpResponse, Responder, Result};
use actix_web::{web, web::Data, App, HttpServer};
use std::path::PathBuf;

use rust_server::database::{Database, InMemoryDb};
use rust_server::inmemory_html_server::InMemoryHtml;
use rust_server::routes::{
    get_login, get_logout_user, get_register, get_user_dashboard,
    get_user_dashboard_template, put_login_user, put_register_user,
};
use rust_server::security::{authorize_with_cookie, Role};

fn make_session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
        .cookie_name(String::from("session"))
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_http_only(true)
        .build()
}

async fn index(in_memory_html: Data<InMemoryHtml>, session: Session) -> Result<impl Responder> {
    let cookie_result = session.get::<String>("session");

    let (has_auth, _) = match cookie_result {
        Ok(cookie_option) => authorize_with_cookie((Role::User, cookie_option)).await,
        Err(_) => (false, "".to_string()),
    };

    let output = in_memory_html.get("index.html".to_string(), has_auth).await;

    match output {
        Some(html) => {
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html))
        }
        None => return Ok(HttpResponse::NotFound().body("Not found")),
    }
}

async fn static_files(req: HttpRequest) -> Result<fs::NamedFile> {
    let mut path = PathBuf::new();
    path.push("../paywall_blog/_site/");

    let path_suffix: PathBuf = req.match_info().query("filename").parse().unwrap();
    path.push(path_suffix);
    Ok(fs::NamedFile::open(path)?)
}

async fn html_files(
    req: HttpRequest,
    in_memory_html: Data<InMemoryHtml>,
    session: Session
) -> Result<impl Responder> {
    let cookie_result = session.get::<String>("session");

    let (has_auth, _) = match cookie_result {
        Ok(cookie_option) => authorize_with_cookie((Role::User, cookie_option)).await,
        Err(_) => (false, "".to_string()),
    };
    let path: String = req.match_info().query("filename").parse().unwrap();

    let output = in_memory_html.get(path, has_auth).await;

    match output {
        Some(html) => {
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html))
        }
        None => return Ok(HttpResponse::NotFound().body("Not found")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_arc: Arc<dyn Database> = Arc::new(InMemoryDb::new());
    let db = Data::from(db_arc);

    let in_memory_html = Data::new(InMemoryHtml::new("../paywall_blog/_site"));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .app_data(in_memory_html.clone())
            .wrap(Logger::default())
            .wrap(make_session_middleware())
            .route("/", web::get().to(index))
            .route("/{filename:[0-9a-zA-Z_\\.-]+\\.(?:js|css|jpg|jpeg|json)$}", web::get().to(static_files)) //files in main folder
            .route("/{filename:[0-9a-zA-Z_\\.-]+\\.html$}", web::get().to(html_files))
            .route("/{filename:(?:posts|images)\\/[0-9a-zA-Z_\\.-]+\\.(?:js|css|jpg|jpeg|json)$}", web::get().to(static_files)) //files in sub-folders
            .route("/{filename:(?:posts|images)\\/[0-9a-zA-Z_\\.-]+\\.html$}", web::get().to(html_files)) //files in sub-folders
            .route("/{filename:site_libs\\/[0-9a-zA-Z_\\.-]+\\/[0-9a-zA-Z_\\.-]+\\.(?:js|css|jpg|jpeg)$}", web::get().to(static_files)) //styles and packages from quarto
            .route("/{filename:site_libs\\/bootstrap/bootstrap-icons.[0-9a-z\\?]+$}", web::get().to(static_files))
            .route("/auth/user-dashboard", web::get().to(get_user_dashboard))
            .route("/auth/user-dashboard-template", web::get().to(get_user_dashboard_template))
            .route("/auth/register", web::get().to(get_register))
            .route("/auth/login", web::get().to(get_login))
            .route("/auth/register-user", web::post().to(put_register_user))
            .route("/auth/login-user", web::post().to(put_login_user))
            .route("/auth/logout-user", web::get().to(get_logout_user))
    }) //weird bootstrap icons files

            .bind(("0.0.0.0", 5001))?
            .run()
            .await
}
