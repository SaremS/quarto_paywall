use log::{error, info};

use actix_session::Session;
use actix_web::{
    web::{Bytes, Data, Json},
    HttpRequest, HttpResponse, Responder, Result,
};

use crate::database::Database;
use crate::envvars::EnvVarLoader;
use crate::inmemory_html_server::InMemoryHtml;
use crate::models::PurchaseIntent;
use crate::purchase::PurchaseHandler;
use crate::security::session_status_from_session;
use crate::utils::ResultOrInfo;

//https://github.com/arlyon/async-stripe/blob/master/examples/webhook-actix.rs
pub async fn stripe_webhook_add_article(
    req: HttpRequest,
    payload: Bytes,
    db: Data<dyn Database>,
    purchase_handler: Data<dyn PurchaseHandler>,
) -> Result<impl Responder> {
    let reference_result = purchase_handler.webhook_to_purchase_reference(&req, &payload);

    match reference_result {
        ResultOrInfo::Ok(purchase_reference) => {
            let db_write_result = db
                .add_accessible_article_to_id(
                    purchase_reference.user_id.clone(),
                    purchase_reference.article.clone(),
                )
                .await;
            match db_write_result {
                Ok(()) => (),
                Err(()) => error!("Error writing to DB"),
            }
        }
        ResultOrInfo::Err(err) => error!("{:?}", err.to_string()),
        ResultOrInfo::Info(info) => info!("{:?}", info),
    }

    return Ok(HttpResponse::Ok());
}

pub async fn stripe_checkout(
    session: Session,
    purchase_intent_json: Json<PurchaseIntent>,
    env_var_loader: Data<EnvVarLoader>,
    html_paywall: Data<InMemoryHtml>,
    purchase_handler: Data<dyn PurchaseHandler>,
) -> Result<impl Responder> {
    let purchase_intent = purchase_intent_json.into_inner();
    let session_status =
        session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;
    let user_id = session_status.user_id.unwrap();

    let checkout_result = purchase_handler
        .checkout(&user_id, &purchase_intent, html_paywall)
        .await;

    match checkout_result {
        Ok(stripe_checkout_url) => return Ok(Json(stripe_checkout_url)),
        Err(err) => return Err(actix_web::error::ErrorBadRequest(err.to_string())),
    }
}
