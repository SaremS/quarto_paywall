use async_trait::async_trait;

use crate::errors::AuthenticationError;
use crate::errors::SignupError;
use crate::models::{RegisterUser, LoginUser, UserCreated, UserLoggedIn, User, PaywallArticle};

#[async_trait]
pub trait Database: Send + Sync {
    async fn create_user(&self, user: RegisterUser) -> Result<UserCreated, SignupError>;
    async fn create_admin(&self, user: RegisterUser) -> Result<UserCreated, SignupError>;
    async fn login(&self, user: LoginUser) -> Result<UserLoggedIn, AuthenticationError>;

    async fn get_user_by_id(&self, id: usize) -> Option<User>;
    async fn add_accessible_article_to_id(&self, id: usize, article: PaywallArticle) -> Result<(),()>;
    async fn user_id_has_access_by_link(&self, id: usize, link: &str) -> bool;
    async fn confirm_email_for_user_id(&self, id: usize) -> Result<(),()>;
    async fn delete_user_by_id(&self, id: usize) -> Result<(),()>;
    async fn get_paywall_articles_for_user_id(&self, id: usize) -> Option<Vec<PaywallArticle>>;
}
