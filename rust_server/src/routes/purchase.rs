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
use crate::models::{ClientReference, PurchaseIntent};
use crate::security::{session_status_from_session, xor_cipher};

//https://github.com/arlyon/async-stripe/blob/master/examples/webhook-actix.rs
pub async fn stripe_webhook_add_article(
    req: HttpRequest,
    payload: Bytes,
    db: Data<dyn Database>,
) -> Result<impl Responder> {
    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();
    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    let stripe_endpoint_key =
        std::env::var("STRIPE_ENDPOINT_KEY").expect("Missing STRIPE_ENDPOINT_KEY in env");

    if let Ok(event) = Webhook::construct_event(payload_str, stripe_signature, &stripe_endpoint_key)
    {
        match event.type_ {
            EventType::AccountUpdated => {
                if let EventObject::Account(account) = event.data.object {
                    handle_account_updated(&account).unwrap();
                }
            }
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    let reference_json = xor_cipher(&session.client_reference_id.unwrap(), 123);
                    let client_reference: ClientReference =
                        serde_json::from_str(&reference_json).unwrap();
                    let _ = db
                        .add_accessible_article_to_id(
                            client_reference.user_id.clone(),
                            client_reference.target.clone(),
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
    req: HttpRequest,
    session: Session,
    intent: Json<PurchaseIntent>,
) -> Result<impl Responder> {
    let session_status = session_status_from_session(&session, &req).await;
    let user_id = session_status.user_id.unwrap();
    let target = (intent.into_inner()).purchase_target;

    let reference = ClientReference { user_id, target };
    let stripe_checkout_url =
        get_stripe_checkout_url(reference, "Article: Paywalled".to_string(), 250).await;

    let response = Json(stripe_checkout_url);

    return Ok(response);
}

async fn get_stripe_checkout_url(
    client_reference: ClientReference,
    name: String,
    price: i64,
) -> String {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let product = {
        let mut create_product = CreateProduct::new(&name);
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
        create_price.unit_amount = Some(price);
        create_price.expand = &["product"];
        Price::create(&client, create_price).await.unwrap()
    };

    println!(
        "created a product {:?} at price {} {}",
        product.name.unwrap(),
        price.unit_amount.unwrap() / 100,
        price.currency.unwrap()
    );

    let reference = serde_json::to_string(&client_reference).unwrap();
    let reference_encoded = xor_cipher(&reference, 123);

    let checkout_session = {
        let mut params = CreateCheckoutSession::new("http://sarem-seitz.com");
        params.cancel_url = Some("http://sarem-seitz.com");
        params.client_reference_id = Some(&reference_encoded);
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