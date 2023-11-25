use actix_session::Session;
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder, Result, HttpRequest
};
use askama::Template;

use crate::database::Database;
use crate::envvars::EnvVarLoader;
use crate::mail::{send_confirmation_mail, send_deletion_mail};
use crate::models::{AuthLevel, LoginUser, MailEnvVars, RegisterUser};
use crate::security::session_status_from_session;
use crate::templates::{
    DeleteUserConfirmedTemplate, DeleteUserTemplate, LoginSuccessTemplate, LoginTemplate,
    LogoutSuccessTemplate, RegisterSuccessTemplate, RegisterTemplate, UberTemplate,
    UserDashboardTemplate,
};

pub async fn get_user_dashboard(
    db: Data<dyn Database>,
    session: Session,
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let session_status =
        session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;
    let auth_level = session_status.auth_level;

    let content;
    if auth_level > AuthLevel::NoAuth {
        let paywall_articles = db.get_paywall_articles_for_user_id(session_status.user_id.unwrap()).await;

        content = UserDashboardTemplate {
            username: session_status.username.unwrap(),
            articles: paywall_articles.unwrap(),
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

pub async fn get_user_dashboard_template(
    session: Session,
    env_var_loader: Data<EnvVarLoader>,
    db: Data<dyn Database>,
) -> Result<impl Responder> {
    let session_status =
        session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;

    let content;

    if session_status.auth_level > AuthLevel::NoAuth {
        let paywall_articles = db.get_paywall_articles_for_user_id(session_status.user_id.unwrap()).await;

        content = UserDashboardTemplate {
            username: session_status.username.unwrap(),
            articles: paywall_articles.unwrap(),
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

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(content);

    return Ok(response);
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
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let create_user_result = db.create_user(user.into_inner()).await;

    let result_content;
    match create_user_result {
        Ok(user_created) => {
            let mail_environment = MailEnvVars {
                mail_secret_key: &env_var_loader.get_mail_secret_key(),
                deletion_secret_key: &env_var_loader.get_deletion_secret_key(),
                smtp_mail_address: &env_var_loader.get_smtp_mail_address(),
                domain_url: &env_var_loader.get_domain_url(),
                smtp_host: &env_var_loader.get_smtp_host(),
                smtp_sender_name: &env_var_loader.get_smtp_sender_name(),
                smtp_username: &env_var_loader.get_smtp_username(),
                smtp_password: &env_var_loader.get_smtp_password(),
            };

            send_confirmation_mail(
                &user_created.user_id,
                &user_created.email,
                &mail_environment,
            )
            .await;

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

pub async fn get_delete_user() -> Result<impl Responder> {
    let delete_user_template = DeleteUserTemplate {}.render().unwrap();

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(delete_user_template);

    return Ok(response);
}

pub async fn get_delete_user_confirmed(
    session: Session,
    db: Data<dyn Database>,
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let session_status =
        session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;

    let mail_environment = MailEnvVars {
        mail_secret_key: &env_var_loader.get_mail_secret_key(),
        deletion_secret_key: &env_var_loader.get_deletion_secret_key(),
        smtp_mail_address: &env_var_loader.get_smtp_mail_address(),
        domain_url: &env_var_loader.get_domain_url(),
        smtp_host: &env_var_loader.get_smtp_host(),
        smtp_sender_name: &env_var_loader.get_smtp_sender_name(),
        smtp_username: &env_var_loader.get_smtp_username(),
        smtp_password: &env_var_loader.get_smtp_password(),
    };

    let user_id = session_status.user_id.unwrap();
    let user = db.get_user_by_id(user_id).await;
    let email = user.unwrap().email;

    send_deletion_mail(&user_id, &email, &mail_environment).await;

    let delete_user_template_confirmed = DeleteUserConfirmedTemplate {}.render().unwrap();

    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(delete_user_template_confirmed);

    return Ok(response);
}
