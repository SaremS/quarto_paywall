use std::collections::HashMap;

use actix_web::{
    web::{Bytes, Data},
    HttpRequest,
};
use async_trait::async_trait;

use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreatePrice, CreateProduct, Currency, EventObject, EventType,
    IdOrCreate, Price, PriceId, Product, ProductId, Webhook,
};

use crate::envvars::EnvVarLoader;
use crate::models::{PaywallArticle, PurchaseIntent, PurchaseReference};
use crate::purchase::{PurchaseError, PurchaseHandler};
use crate::utils::ResultOrInfo;

pub struct StripePurchaseHandler {
    stripe_webhook_key: String,
    stripe_secret_key: String,
    domain_url: String,
}

#[async_trait]
impl PurchaseHandler for StripePurchaseHandler {
    async fn checkout(
        &self,
        user_id: &usize,
        purchase_intent: &PurchaseIntent,
        article: &PaywallArticle,
    ) -> Result<String, PurchaseError> {
        let target_domainpath = self.domain_url.clone() + &purchase_intent.purchase_target;
        let reference = PurchaseReference {
            user_id: user_id.clone(),
            article: article.clone(),
        };
        let stripe_checkout_url = self
            .get_stripe_checkout_url(&reference, &target_domainpath)
            .await;
        return Ok(stripe_checkout_url);
    }

    fn webhook_to_purchase_reference(
        &self,
        req: &HttpRequest,
        payload: &Bytes,
    ) -> ResultOrInfo<PurchaseReference, PurchaseError, String> {
        let payload_str = std::str::from_utf8(payload).unwrap();
        let stripe_signature = self
            .get_header_value(req, "Stripe-Signature")
            .unwrap_or_default();

        if let Ok(event) =
            Webhook::construct_event(payload_str, stripe_signature, &self.stripe_webhook_key)
        {
            match event.type_ {
                EventType::CheckoutSessionCompleted => {
                    if let EventObject::CheckoutSession(session) = event.data.object {
                        let reference_json = session.client_reference_id.unwrap();
                        let purchase_reference: PurchaseReference =
                            serde_json::from_str(&reference_json).unwrap();

                        return ResultOrInfo::Ok(purchase_reference);
                    } else {
                        return ResultOrInfo::Err(PurchaseError::StripeEventDataNotFoundError);
                    }
                }
                _ => {
                    return ResultOrInfo::Info(format!("Non checkout event: {:?}", event.type_));
                }
            }
        } else {
            return ResultOrInfo::Err(PurchaseError::StripeWebhookEventError);
        }
    }
}

impl StripePurchaseHandler {
    pub fn new_from_envvars(env_var_loader: &EnvVarLoader) -> StripePurchaseHandler {
        return StripePurchaseHandler {
            stripe_webhook_key: env_var_loader.get_stripe_webhook_key(),
            stripe_secret_key: env_var_loader.get_stripe_secret_key(),
            domain_url: env_var_loader.get_domain_url(),
        };
    }

    fn get_header_value<'b>(&self, req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
        return req.headers().get(key)?.to_str().ok();
    }

    async fn get_stripe_checkout_url(
        &self,
        purchase_reference: &PurchaseReference,
        domainpath: &str,
    ) -> String {
        let client = Client::new(self.stripe_secret_key.clone());
        let product =
            StripePurchaseHandler::create_stripe_product(&client, purchase_reference).await;
        let price =
            StripePurchaseHandler::create_stripe_price(&client, purchase_reference, &product.id)
                .await;
        let checkout_session = StripePurchaseHandler::create_stripe_checkout_session(
            &purchase_reference,
            domainpath,
            &price.id,
            &client,
        )
        .await;

        return checkout_session.url.unwrap();
    }

    async fn create_stripe_product(
        client: &Client,
        purchase_reference: &PurchaseReference,
    ) -> Product {
        let mut create_product = CreateProduct::new(&purchase_reference.article.title);
        create_product.metadata = Some(HashMap::from([(
            String::from("async-stripe"),
            String::from("true"),
        )]));

        let product = Product::create(client, create_product).await.unwrap();
        return product;
    }

    async fn create_stripe_price(
        client: &Client,
        purchase_reference: &PurchaseReference,
        product_id: &ProductId,
    ) -> Price {
        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(IdOrCreate::Id(product_id));
        create_price.metadata = Some(HashMap::from([(
            String::from("async-stripe"),
            (String::from("true")),
        )]));

        create_price.unit_amount = Some(purchase_reference.article.get_price_in_minor_unit());
        create_price.expand = &["product"];
        let price = Price::create(client, create_price).await.unwrap();
        return price;
    }

    async fn create_stripe_checkout_session(
        purchase_reference: &PurchaseReference,
        domainpath: &str,
        price_id: &PriceId,
        client: &Client,
    ) -> CheckoutSession {
        let reference = serde_json::to_string(purchase_reference).unwrap();
        let success_path = domainpath.to_string() + "?success=1";
        let cancel_path = domainpath.to_string() + "?success=0";

        let mut params = CreateCheckoutSession::new(&success_path);
        params.cancel_url = Some(&cancel_path);
        params.client_reference_id = Some(&reference);
        params.mode = Some(CheckoutSessionMode::Payment);
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            quantity: Some(1),
            price: Some(price_id.to_string()),
            ..Default::default()
        }]);
        params.expand = &["line_items", "line_items.data.price.product"];

        let checkout_session = CheckoutSession::create(client, params).await.unwrap();
        return checkout_session;
    }
}
