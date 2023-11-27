use actix_web::{
    web::{Data, Query},
    HttpResponse, Responder, Result,
};
use actix_session::Session;
use serde::Deserialize;

use crate::database::Database;
use crate::user_communication::VerifyAndDeleteUser;

#[derive(Deserialize)]
pub struct VerifyUserQuery {
    token: String,
}

pub async fn confirm_user(
    query: Query<VerifyUserQuery>,
    db: Data<dyn Database>,
    verifier: Data<dyn VerifyAndDeleteUser>,
) -> Result<impl Responder> {
    let verification_result = verifier.handle_registration_verification(&query.token).await;

    match verification_result {
        Ok(user_id) => {
            let _ = db
                .confirm_email_for_user_id(user_id)
                .await;
            let body = "Confirmation successful - you can now close this page";

            let response = HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body);

            return Ok(response);
        }
        Err(error) => {
            let body = error.to_string();
            let response = HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body);

            return Ok(response);
        }
    }
}

pub async fn delete_user(
    query: Query<VerifyUserQuery>,
    db: Data<dyn Database>,
    verifier: Data<dyn VerifyAndDeleteUser>,
    session: Session
) -> Result<impl Responder> {
    let verification_result = verifier.handle_deletion_verification(&query.token).await;

    match verification_result {
        Ok(user_id) => {
            let _ = db.delete_user_by_id(user_id).await;
            let body = "Deletion successful - you can now close this page";
            let _ = session.remove("session");

            let response = HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body);

            return Ok(response);
        }
        Err(error) => {
            let body = error.to_string();
            let response = HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body);

            return Ok(response);
        }
    }
}
