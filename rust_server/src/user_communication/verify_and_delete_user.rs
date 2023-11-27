use async_trait::async_trait;
use thiserror::Error;

#[async_trait]
pub trait VerifyAndDeleteUser: Send + Sync {
    async fn send_registration_verification(&self, user_id: &usize, recipient: &str);
    async fn handle_registration_verification(
        &self,
        token: &str,
    ) -> Result<usize, VerificationError>;

    async fn send_deletion_verification(&self, user_id: &usize, recipient: &str);
    async fn handle_deletion_verification(&self, token: &str) -> Result<usize, VerificationError>;
}

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("There's a problem with your confirmation link - it might have expired. Please request another one.")]
    TokenError,
}
