use actix_web::{
    web::{Data, Query},
    HttpResponse, Responder, Result,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

use crate::database::Database;
use crate::envvars::EnvVarLoader;
use crate::models::EmailConfirmation;

#[derive(Deserialize)]
struct ConfirmUserQuery<'a>{
    token: &'a str,
}

pub async fn confirm_user<'a>(
    query: Query<ConfirmUserQuery<'a>>,
    db: Data<dyn Database>,
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let token = query.token;
    let email_secret = env_var_loader.get_jwt_secret_key(); //TODO: Use independent key

    let decoded = decode::<EmailConfirmation>(
        &token,
        &DecodingKey::from_secret(email_secret.as_bytes()),
        &Validation::default(),
    );

    match decoded {
        Ok(user_confirm) => {
            db.confirm_email_for_user_id(user_confirm.claims.user_id).await;
            let body = "Confirmation successful - you can now close this page";

            let response = HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body);

            return Ok(response);
        },
        Err(_) => {
            let body = "Something went wrong, please request a new confirmation link - you can now close this page";
            let response = HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body);

            return Ok(response);
        }
    }
}
