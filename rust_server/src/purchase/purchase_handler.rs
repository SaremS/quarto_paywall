use actix_web::{
    web::Bytes,
    HttpRequest,
};
use async_trait::async_trait;
use thiserror::Error;

use crate::models::{PurchaseIntent, PurchaseReference, PaywallArticle};
use crate::utils::ResultOrInfo;

#[async_trait]
pub trait PurchaseHandler: Sync + Send {
    async fn checkout(
        &self,
        user_id: &usize,
        purchase_intent: &PurchaseIntent,
        article: &PaywallArticle
    ) -> Result<String, PurchaseError>;
    fn webhook_to_purchase_reference(
        &self,
        req: &HttpRequest,
        payload: &Bytes,
    ) -> ResultOrInfo<PurchaseReference, PurchaseError, String>;
}

#[derive(Error, Debug)]
pub enum PurchaseError {
    #[error("Internal error, the purchase target was not found.")]
    TargetNotFoundError,
    #[error("Error while constructing stripe webhook event.")]
    StripeWebhookEventError,
    #[error("Stripe event data not found")]
    StripeEventDataNotFoundError
}
