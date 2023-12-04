use std::collections::HashMap;

use async_trait::async_trait;

use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreatePrice, CreateProduct, Currency, EventObject, EventType,
    IdOrCreate, Price, PriceId, Product, ProductId, Webhook,
};

use crate::models::PurchaseReference;
use crate::purchase::PurchaseError;
use crate::utils::ResultOrInfo;

#[async_trait]
pub trait AbstractStripeClient: Sync + Send {
    async fn get_stripe_checkout_url(&self, purchase_reference: &PurchaseReference, domainpath: &str) -> String;
    async fn webhook_to_purchase_reference(
        &self,
        payload: &str,
        signature: &str,
    ) -> ResultOrInfo<PurchaseReference, PurchaseError, String>;
}

pub struct StripeClient {
    stripe_webhook_key: String,
    stripe_secret_key: String,
}

#[async_trait]
impl AbstractStripeClient for StripeClient {
    async fn get_stripe_checkout_url(
        &self,
        purchase_reference: &PurchaseReference,
        domainpath: &str,
    ) -> String {
        let client = Client::new(self.stripe_secret_key.clone());
        let product =
            StripeClient::create_stripe_product(&client, purchase_reference).await;
        let price =
            StripeClient::create_stripe_price(&client, purchase_reference, &product.id)
                .await;
        let checkout_session = StripeClient::create_stripe_checkout_session(
            &purchase_reference,
            domainpath,
            &price.id,
            &client,
        )
        .await;

        return checkout_session.url.unwrap();
    }

    async fn webhook_to_purchase_reference(
        &self,
        payload: &str,
        stripe_signature: &str,
    ) -> ResultOrInfo<PurchaseReference, PurchaseError, String> {
        if let Ok(event) =
            Webhook::construct_event(payload, stripe_signature, &self.stripe_webhook_key)
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

impl StripeClient {
    pub fn new(stripe_webhook_key: &str, stripe_secret_key: &str) -> StripeClient {
        return StripeClient {
            stripe_webhook_key: stripe_webhook_key.to_string(),
            stripe_secret_key: stripe_secret_key.to_string(),
        };
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
