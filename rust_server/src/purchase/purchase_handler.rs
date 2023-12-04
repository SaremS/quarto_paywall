use thiserror::Error;

use crate::models::{PaywallArticle, PurchaseIntent, PurchaseReference};
use crate::purchase::AbstractStripeClient;
use crate::utils::ResultOrInfo;

pub struct PurchaseHandler {
    domain_url: String,
    stripe_client: Box<dyn AbstractStripeClient>,
}

impl PurchaseHandler {
    pub fn new(domain_url: &str, stripe_client: Box<dyn AbstractStripeClient>) -> PurchaseHandler {
        return PurchaseHandler {
            domain_url: domain_url.to_string(),
            stripe_client
        }; 
    }

    pub async fn stripe_checkout(
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
            .stripe_client
            .get_stripe_checkout_url(&reference, &target_domainpath)
            .await;
        return Ok(stripe_checkout_url);
    }

    pub async fn stripe_webhook_to_purchase_reference(
        &self,
        payload: &str,
        stripe_signature: &str,
    ) -> ResultOrInfo<PurchaseReference, PurchaseError, String> {
        let result = self
            .stripe_client
            .webhook_to_purchase_reference(payload, stripe_signature)
            .await;
        return result;
    }
}

#[derive(Error, Debug)]
pub enum PurchaseError {
    #[error("Internal error, the purchase target was not found.")]
    TargetNotFoundError,
    #[error("Error while constructing stripe webhook event.")]
    StripeWebhookEventError,
    #[error("Stripe event data not found")]
    StripeEventDataNotFoundError,
}
