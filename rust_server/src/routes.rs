use std::{borrow::Borrow, path::PathBuf};

use actix_files as fs;
use actix_session::Session;
use actix_web::{
    web::{Bytes, Data, Json, Redirect},
    HttpRequest, HttpResponse, Responder, Result
};
use askama::Template;
use serde::{Serialize, Deserialize};
use futures::stream::{once, StreamExt};

use stripe::{EventObject, EventType, Webhook, WebhookError};
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCustomer, CreatePrice, CreateProduct, Currency, Customer,
    Expandable, IdOrCreate, Price, Product,
};
use stripe::CustomerId;

use crate::database::Database;
use crate::inmemory_html_server::InMemoryHtml;
use crate::models::{LoginUser, RegisterUser, PurchaseIntent};
use crate::security::{AuthLevel, SessionStatus, xor_cipher};
use crate::templates::{
    LoginSuccessTemplate, LoginTemplate, LogoutSuccessTemplate, RegisterSuccessTemplate,
    RegisterTemplate, UberTemplate, UserDashboardTemplate,
};

pub async fn index(
    in_memory_html: Data<InMemoryHtml>,
    session_status: SessionStatus,
) -> Result<impl Responder> {
    let output = in_memory_html
        .get("index.html".to_string(), session_status)
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

pub async fn html_files(
    req: HttpRequest,
    in_memory_html: Data<InMemoryHtml>,
    session_status: SessionStatus,
) -> Result<impl Responder> {
    let path: String = req.match_info().query("filename").parse().unwrap();

    let output = in_memory_html.get(path, session_status).await;

    match output {
        Some(html) => {

            let body = once(async move { Ok::<_, actix_web::Error>(Bytes::from(html)) });
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .streaming(body))
        }
        None => return Ok(HttpResponse::NotFound().body("Not found")),
    }
}

pub async fn get_user_dashboard(session_status: SessionStatus) -> Result<impl Responder> {
    let auth_level = session_status.auth_level;

    let content;
    if auth_level > AuthLevel::NoAuth {
        content = UserDashboardTemplate {
            username: session_status.username.unwrap(),
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

pub async fn get_user_dashboard_template(session_status: SessionStatus) -> Result<impl Responder> {
    let content;

    if session_status.auth_level > AuthLevel::NoAuth {
        content = UserDashboardTemplate {
            username: session_status.username.unwrap(),
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

pub async fn get_paywalled_content(
    req: HttpRequest,
    session_status: SessionStatus,
) -> Result<fs::NamedFile> {
    let mut path;
    let filename: PathBuf = req.match_info().query("filename").parse().unwrap();

    if session_status.auth_level > AuthLevel::NoAuth {
        path = PathBuf::from("../paywall_blog/paywalled/");
        path.push(filename);
    } else {
        path = PathBuf::from("./templates/paywall.html");
    }

    return Ok(fs::NamedFile::open(path)?);
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
) -> Result<impl Responder> {
    let create_user_result = db.create_user(user.into_inner()).await;

    let result_content;
    match create_user_result {
        Ok(user_created) => {
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

#[derive(Serialize, Deserialize, Debug)]
struct ClientReference {
    user_id: usize,
    target: String
}

//https://github.com/arlyon/async-stripe/blob/master/examples/webhook-actix.rs
pub async fn stripe_webhook_add_article(
    req: HttpRequest,
    payload: Bytes,
    db: Data<dyn Database>,
) -> Result<impl Responder> {
    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();
    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    let stripe_endpoint_key = std::env::var("STRIPE_ENDPOINT_KEY").expect("Missing STRIPE_ENDPOINT_KEY in env");

    if let Ok(event) = Webhook::construct_event(payload_str, stripe_signature, &stripe_endpoint_key) {
        match event.type_ {
            EventType::AccountUpdated => {
                if let EventObject::Account(account) = event.data.object {
                    handle_account_updated(&account).unwrap();
                }
            }
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    let reference_json = xor_cipher(&session.client_reference_id.unwrap(), 123);
                    let client_reference: ClientReference = serde_json::from_str(&reference_json).unwrap();
                    let res = db.add_accessible_article_to_id(client_reference.user_id.clone(), client_reference.target.clone()).await;
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
    println!("Received account updated webhook for account: {:?}", account.id);
    Ok(())
}


pub async fn stripe_checkout(session_status: SessionStatus, intent: Json<PurchaseIntent>) -> Result<impl Responder> {
    let user_id = session_status.user_id.unwrap();
    let target = (intent.into_inner()).purchase_target;

    let reference = ClientReference {user_id, target};
    let stripe_checkout_url = get_stripe_checkout_url(reference, "Article: Paywalled".to_string(), 250).await;
    
    let response = Json(stripe_checkout_url);

    return Ok(response);
}


async fn get_stripe_checkout_url(client_reference: ClientReference, name: String, price: i64) -> String {
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
