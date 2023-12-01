use std::{collections::HashMap,sync::Arc};

use actix_web::{middleware::Logger, web::{Data, get, post}, App, HttpServer};

use rust_server::database::{Database, InMemoryDb};
use rust_server::inmemory_static_files::InMemoryStaticFiles;
use rust_server::routes::{
    auth::{get_login, get_logout_user, get_register, get_user_dashboard, get_user_dashboard_template,
    put_login_user, put_register_user, get_delete_user, get_delete_user_confirmed},
    purchase::{stripe_checkout, stripe_webhook_add_article},
    static_files::{html_files, index, static_files, in_memory_static_files},
    mail::{confirm_user, delete_user}
};
use rust_server::security::{make_session_middleware, ScryptHashing};
use rust_server::models::RegisterUser;
use rust_server::envvars::EnvVarLoader;
use rust_server::user_communication::{EmailDevice, VerifyAndDeleteUser};
use rust_server::purchase::{PurchaseHandler, StripePurchaseHandler};
use rust_server::paywall::{make_quarto_paywall, AuthLevelConditionalObject, PaywallItem};


type HashMapServer = HashMap<String, PaywallItem<String, AuthLevelConditionalObject<String>>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_var_loader = EnvVarLoader::new();

    let db_base: InMemoryDb<ScryptHashing> = InMemoryDb::new(env_var_loader.get_jwt_secret_key());

    let admin_user = RegisterUser {
        email: env_var_loader.get_admin_email(),
        username: "admin".to_string(),
        password: env_var_loader.get_admin_password(),
        password_repeat: env_var_loader.get_admin_password() 
    };

    let _  = db_base.create_admin(admin_user).await;
    let db_arc: Arc<dyn Database> = Arc::new(db_base);
    let db = Data::from(db_arc);
 
    let in_memory_static = Data::new(InMemoryStaticFiles::new(&env_var_loader.get_path_static_files()));

    let mail_verifier = EmailDevice::new_from_envvars(&env_var_loader);
    let mail_verifier_arc: Arc<dyn VerifyAndDeleteUser> = Arc::new(mail_verifier);
    let mail_verifier_data = Data::from(mail_verifier_arc);

    let purchase_handler = StripePurchaseHandler::new_from_envvars(&env_var_loader);
    let purchase_handler_arc: Arc<dyn PurchaseHandler> = Arc::new(purchase_handler);
    let purchase_handler_data = Data::from(purchase_handler_arc);

    let quarto_paywall = make_quarto_paywall::<HashMapServer>(&env_var_loader.get_path_static_files());
    let quarto_paywall_arc = Data::from(Arc::new(quarto_paywall));

    let env_var_data = Data::from(Arc::new(env_var_loader));

    

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .app_data(env_var_data.clone())
            .app_data(mail_verifier_data.clone())
            .app_data(in_memory_static.clone())
            .app_data(purchase_handler_data.clone())
            .app_data(quarto_paywall_arc.clone())
            .wrap(Logger::default())
            .wrap(make_session_middleware())
            .route("/", get().to(index::<HashMapServer>))
            .route("/{filename:[0-9a-zA-Z_\\.-]+\\.(?:js|css|jpg|jpeg|json)$}", get().to(static_files)) //files in main folder
            .route("/{filename:[0-9a-za-z_\\.-]+\\.html$}", get().to(html_files::<HashMapServer>))
            .route("/{filename:(?:posts|images)\\/[0-9a-za-z_\\.-]+\\.(?:jpg|jpeg|json)$}", get().to(static_files)) //files in sub-folders
            .route("/{filename:(?:posts|images)\\/[0-9a-za-z_\\.-]+\\.(?:js|css)$}", get().to(in_memory_static_files)) //files in sub-folders

            .route("/{filename:(?:posts|images)\\/[0-9a-zA-Z_\\.-]+\\.html$}", get().to(html_files::<HashMapServer>)) //files in sub-folders
            .route("/{filename:site_libs\\/[0-9a-zA-Z_\\.-]+\\/[0-9a-zA-Z_\\.-]+\\.(?:js|css|jpg|jpeg)$}", get().to(static_files)) //styles and packages from quarto
            .route("/{filename:site_libs\\/bootstrap/bootstrap-icons.[0-9a-z\\?]+$}", get().to(static_files))
            .route("/auth/register", get().to(get_register))
            .route("/auth/login", get().to(get_login))
            .route("/auth/register-user", post().to(put_register_user))
            .route("/auth/login-user", post().to(put_login_user))
            .route("/auth/logout-user", get().to(get_logout_user))
            .route("/auth/user-dashboard", get().to(get_user_dashboard))
            .route("/purchase/checkout", post().to(stripe_checkout::<HashMapServer>))
            .route("/purchase/stripe-webhook", post().to(stripe_webhook_add_article))
            .route("/auth/user-dashboard-template", get().to(get_user_dashboard_template))
            .route("/confirm-user", get().to(confirm_user))
            .route("/delete-user", get().to(delete_user))
            .route("/auth/delete-user", get().to(get_delete_user))
            .route("/auth/delete-user-confirmed", get().to(get_delete_user_confirmed))
    }) 
            .bind(("0.0.0.0", 5001))?
            .run()
            .await
}
