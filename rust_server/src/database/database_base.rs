use async_trait::async_trait;

use crate::errors::AuthenticationError;
use crate::errors::SignupError;
use crate::models::{RegisterUser, LoginUser, UserCreated, UserLoggedIn};

#[async_trait]
pub trait Database: Send + Sync {
    async fn create_user(&self, user: RegisterUser) -> Result<UserCreated, SignupError>;
    async fn login(&self, user: LoginUser) -> Result<UserLoggedIn, AuthenticationError>;
}
