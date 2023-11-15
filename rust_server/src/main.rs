use std::sync::Arc;

use actix_web::{middleware::Logger, web::{Data, get, post}, App, HttpServer};

use rust_server::database::{Database, InMemoryDb};
use rust_server::inmemory_html_server::InMemoryHtml;
use rust_server::routes::{
    get_login, get_logout_user, get_register, get_user_dashboard, get_user_dashboard_template,
    html_files, index, put_login_user, put_register_user, static_files,
};
use rust_server::security::{AuthCheck, make_session_middleware};
use rust_server::models::RegisterUser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_base = InMemoryDb::new();
    let admin_user = RegisterUser {
        email: "admin@admin.com".to_string(),
        username: "admin".to_string(),
        password: "asdf".to_string(),
        password_repeat: "asdf".to_string()
    };

    let _  = db_base.create_admin(admin_user).await;
    let db_arc: Arc<dyn Database> = Arc::new(db_base);
   
    let db = Data::from(db_arc);
 

    let in_memory_html = Data::new(InMemoryHtml::new("../paywall_blog/_site"));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .app_data(in_memory_html.clone())
            .wrap(AuthCheck::new())
            .wrap(Logger::default())
            .wrap(make_session_middleware())
            .route("/", get().to(index))
            .route("/{filename:[0-9a-zA-Z_\\.-]+\\.(?:js|css|jpg|jpeg|json)$}", get().to(static_files)) //files in main folder
            .route("/{filename:[0-9a-zA-Z_\\.-]+\\.html$}", get().to(html_files))
            .route("/{filename:(?:posts|images)\\/[0-9a-zA-Z_\\.-]+\\.(?:js|css|jpg|jpeg|json)$}", get().to(static_files)) //files in sub-folders
            .route("/{filename:(?:posts|images)\\/[0-9a-zA-Z_\\.-]+\\.html$}", get().to(html_files)) //files in sub-folders
            .route("/{filename:site_libs\\/[0-9a-zA-Z_\\.-]+\\/[0-9a-zA-Z_\\.-]+\\.(?:js|css|jpg|jpeg)$}", get().to(static_files)) //styles and packages from quarto
            .route("/{filename:site_libs\\/bootstrap/bootstrap-icons.[0-9a-z\\?]+$}", get().to(static_files))
            .route("/auth/user-dashboard", get().to(get_user_dashboard))
            .route("/auth/user-dashboard-template", get().to(get_user_dashboard_template))
            .route("/auth/register", get().to(get_register))
            .route("/auth/login", get().to(get_login))
            .route("/auth/register-user", post().to(put_register_user))
            .route("/auth/login-user", post().to(put_login_user))
            .route("/auth/logout-user", get().to(get_logout_user))
    }) 
            .bind(("0.0.0.0", 5001))?
            .run()
            .await
}
