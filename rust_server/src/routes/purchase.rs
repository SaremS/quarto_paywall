use log::info;
use std::borrow::Borrow;

use actix_session::Session;
use actix_web::{
    web::{Bytes, Data, Json},
    HttpRequest, HttpResponse, Responder, Result,
};
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreatePrice, CreateProduct, Currency, IdOrCreate, Price,
    Product,
};
use stripe::{EventObject, EventType, Webhook, WebhookError};

use crate::database::Database;
use crate::envvars::EnvVarLoader;
use crate::inmemory_html_server::InMemoryHtml;
use crate::models::{PurchaseReference, PurchaseIntent};
use crate::security::session_status_from_session;

//https://github.com/arlyon/async-stripe/blob/master/examples/webhook-actix.rs
pub async fn stripe_webhook_add_article(
    req: HttpRequest,
    payload: Bytes,
    db: Data<dyn Database>,
    env_var_loader: Data<EnvVarLoader>,
) -> Result<impl Responder> {
    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();
    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    let stripe_webhook_key = env_var_loader.get_stripe_webhook_key();

    if let Ok(event) = Webhook::construct_event(payload_str, stripe_signature, &stripe_webhook_key)
    {
        match event.type_ {
            EventType::AccountUpdated => {
                if let EventObject::Account(account) = event.data.object {
                    handle_account_updated(&account).unwrap();
                }
            }
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    let reference_json = session.client_reference_id.unwrap();
                    let purchase_reference: PurchaseReference =
                        serde_json::from_str(&reference_json).unwrap();
                    let _ = db
                        .add_accessible_article_to_id(
                            purchase_reference.user_id.clone(),
                            purchase_reference.article.clone(),
                        )
                        .await;
                }
            }
            _ => {
                println!("Unknown event encountered in webhook: {:?}", event.type_);
            }
        }
    } else {
        println!("Failed to construct webhook event, ensure your webhook secret is correct.");
    }

    return Ok(HttpResponse::Ok());
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
    return req.headers().get(key)?.to_str().ok();
}

fn handle_account_updated(account: &stripe::Account) -> Result<(), WebhookError> {
    println!(
        "Received account updated webhook for account: {:?}",
        account.id
    );
    Ok(())
}

pub async fn stripe_checkout(
    session: Session,
    intent: Json<PurchaseIntent>,
    env_var_loader: Data<EnvVarLoader>,
    html_paywall: Data<InMemoryHtml>,
) -> Result<impl Responder> {
    let target_path = (intent.into_inner()).purchase_target;

    if let Some(article) = html_paywall.get_paywall_data(&target_path).await {
        let session_status =
            session_status_from_session(&session, &env_var_loader.get_jwt_secret_key()).await;
        let user_id = session_status.user_id.unwrap();

        let target_domainpath = env_var_loader.get_domain_url() + &target_path;

        let reference = PurchaseReference { user_id, article };
        let stripe_checkout_url = get_stripe_checkout_url(
            reference,
            env_var_loader.get_stripe_secret_key(),
            target_domainpath,
        )
        .await;

        let response = Json(stripe_checkout_url);

        return Ok(response);
    } else {
        return Err(actix_web::error::ErrorBadRequest("Target article not found"));
    }
}

async fn get_stripe_checkout_url(
    purchase_reference: PurchaseReference,
    stripe_secret_key: String,
    domainpath: String,
) -> String {
    let client = Client::new(stripe_secret_key);

    let product = {
        let mut create_product = CreateProduct::new(&purchase_reference.article.title);
        create_product.metadata = Some(std::collections::HashMap::from([(
            String::from("async-stripe"),
            String::from("true"),
        )]));
        Product::create(&client, create_product).await.unwrap()
    };

    // and add a price for it in USD
    let price = {
        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(IdOrCreate::Id(&product.id));
        create_price.metadata = Some(std::collections::HashMap::from([(
            String::from("async-stripe"),
            String::from("true"),
        )]));
        create_price.unit_amount = Some(purchase_reference.article.get_price_in_minor_unit());
        create_price.expand = &["product"];
        Price::create(&client, create_price).await.unwrap()
    };

    info!(
        "created a product {:?} at price {} {}",
        product.name.unwrap(),
        price.unit_amount.unwrap() / 100,
        price.currency.unwrap()
    );

    let reference = serde_json::to_string(&purchase_reference).unwrap();

    let success_path = domainpath.clone() + "?success=1";
    let cancel_path = domainpath + "?success=0";

    let checkout_session = {
        let mut params = CreateCheckoutSession::new(&success_path);
        params.cancel_url = Some(&cancel_path);
        params.client_reference_id = Some(&reference);
        params.mode = Some(CheckoutSessionMode::Payment);
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            quantity: Some(1),
            price: Some(price.id.to_string()),
            ..Default::default()
        }]);
        params.expand = &["line_items", "line_items.data.price.product"];

        CheckoutSession::create(&client, params).await.unwrap()
    };

    return checkout_session.url.unwrap();
}
