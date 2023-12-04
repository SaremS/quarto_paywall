extern crate rust_server;

use async_trait::async_trait;

use rust_server::purchase::{AbstractStripeClient, PurchaseError};
use rust_server::user_communication::AbstractEmailClient;
use rust_server::models::PurchaseReference;
use rust_server::utils::ResultOrInfo;

pub struct MockStripeClient<'a>{
    pub expected_purchase_reference: PurchaseReference,
    pub expected_domainpath: &'a str,
    pub expected_payload: &'a str,
    pub expected_signature: &'a str
}

#[async_trait]
impl<'a> AbstractStripeClient for MockStripeClient<'a> {
    async fn get_stripe_checkout_url(&self, purchase_reference: &PurchaseReference, domainpath: &str) -> String {
        assert_eq!(&self.expected_purchase_reference, purchase_reference);
        assert_eq!(self.expected_domainpath, domainpath);

        return "Output".to_string();
    }

    async fn webhook_to_purchase_reference(&self, payload: &str, signature: &str) -> ResultOrInfo<PurchaseReference, PurchaseError, String> {
        assert_eq!(self.expected_payload, payload);
        assert_eq!(self.expected_signature, signature);

        let result = self.expected_purchase_reference.clone();

        return ResultOrInfo::Ok(result);
    }
}

pub struct MockEmailClient {
    pub expected_recipient: String,
    pub expected_subject: String,
    pub expected_body: String 
}

#[async_trait]
impl<'a> AbstractEmailClient for MockEmailClient {  
    async fn send(&self, recipient: &str, subject: &str, body: &str) -> Result<(),()> {
        assert_eq!(self.expected_recipient, recipient);
        assert_eq!(self.expected_subject, subject);

        return Ok(());
    }
}
