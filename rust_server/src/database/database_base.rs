use async_trait::async_trait;

use crate::errors::AuthenticationError;
use crate::errors::SignupError;
use crate::models::{LoginUser, PaywallArticle, RegisterUser, User, UserCreated, UserLoggedIn};

#[async_trait]
pub trait Database: Send + Sync {
    ///Create new user and directly provide login JWT after successful signup
    ///```
    ///use tokio::runtime::Runtime;
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::models::RegisterUser;
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///     };
    ///
    ///let rt = Runtime::new().unwrap();
    ///let registered = rt.block_on(db.create_user(new_user)).unwrap();
    ///assert_eq!(registered.user_id, 0);
    ///assert_eq!(registered.email, "test@test.com");
    ///assert_eq!(registered.username, "test");
    ///```
    async fn create_user(&self, user: RegisterUser) -> Result<UserCreated, SignupError>;

    ///Enter user email and receive user object
    ///```
    ///use tokio::runtime::Runtime;
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::models::RegisterUser;
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///     };
    ///
    ///let rt = Runtime::new().unwrap();
    ///let registered = rt.block_on(db.create_user(new_user)).unwrap();
    ///let user = rt.block_on(db.get_user_by_email("test@test.com")).await.unwrap();
    ///assert_eq!(user.email, new_user.email);
    ///assert_eq!(user.username, new_user.username);
    ///```
    async fn get_user_by_email(&self, email: &str) -> Option<User>;

    ///Verify that a certain user email exists
    ///```
    ///use tokio::runtime::Runtime;
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::models::RegisterUser;
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///     };
    ///
    ///let rt = Runtime::new().unwrap();
    ///let registered = rt.block_on(db.create_user(new_user)).unwrap();
    ///
    ///let user_exists = rt.block_on(db.check_email_exists("test@test.com")).unwrap();
    ///assert!(user_exists);
    ///
    ///let user_does_not_exist = rt.block_on(db.check_email_exists("not@there.com")).unwrap();
    ///assert!(!user_does_not_exist);
    ///```
    async fn check_email_exists(&self, email: &str) -> bool;

    ///Create user with admin privileges
    async fn create_admin(&self, user: RegisterUser) -> Result<UserCreated, SignupError>;

    ///Match provided credentials against stored credentials and
    ///grant access token if they are matching.
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::models::{RegisterUser, LoginUser};
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///};
    ///
    ///let rt = Runtime::new().unwrap();
    ///let _ = rt.block_on(db.create_user(new_user));
    ///
    ///let login_user = LoginUser {
    ///     email: "test@test.com".to_string(),
    ///     password: "insecure password".to_string()
    ///};
    ///let logged_in = rt.block_on(db.login(login_user)).unwrap();
    ///assert_eq!(logged_in.username, "test");
    ///```
    async fn login(&self, user: LoginUser) -> Result<UserLoggedIn, AuthenticationError>;

    ///Provide a `user_id` and get the corresponding `User` object
    ///if it exists. If not, returns `None`.
    ///
    ///Primarily used for matching a JWT user id to the respective user
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::models::{RegisterUser, LoginUser};
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///};
    ///
    ///let rt = Runtime::new().unwrap();
    ///let _ = rt.block_on(db.create_user(new_user));
    ///
    ///let user_option = rt.block_on(db.get_user_by_id(1));
    ///assert!(user_option.is_none());
    ///
    ///let user = rt.block_on(db.get_user_by_id(0)).unwrap();
    ///assert_eq!(user.username, "test");
    ///```
    async fn get_user_by_id(&self, id: usize) -> Option<User>;

    ///Add a `PaywallArticle` to the list of `accessible_articles` for a
    ///user with a given `user_id`.
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::models::{RegisterUser, LoginUser, PaywallArticle};
    ///use rust_server::price::Price;
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///};
    ///
    ///let rt = Runtime::new().unwrap();
    ///let _ = rt.block_on(db.create_user(new_user));
    ///
    ///let article = PaywallArticle::new(
    ///     "test identifier".to_string(),
    ///     "/test/test-article".to_string(),
    ///     "test title".to_string(),
    ///     Price::from_currency_string(10, "USD").unwrap()
    ///);
    ///
    ///let add_result = rt.block_on(db.add_accessible_article_to_id(0, article));
    ///
    ///assert!(add_result.is_ok());
    ///```
    async fn add_accessible_article_to_id(
        &self,
        id: usize,
        article: PaywallArticle,
    ) -> Result<(), ()>;

    ///For a given `user_id`, check if there exists a `PaywallArticle`
    ///that contains the provided `link`; in the future, will probably use
    ///`identifier` to check access.
    ///
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::models::{RegisterUser, LoginUser, PaywallArticle};
    ///use rust_server::price::Price;
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///};
    ///
    ///let rt = Runtime::new().unwrap();
    ///let _ = rt.block_on(db.create_user(new_user));
    ///
    ///let article = PaywallArticle::new(
    ///     "test identifier".to_string(),
    ///     "/test/test-article".to_string(),
    ///     "test title".to_string(),
    ///     Price::from_currency_string(10, "USD").unwrap()
    ///);
    ///
    ///let _ = rt.block_on(db.add_accessible_article_to_id(0, article));
    ///
    ///let has_access = rt.block_on(db.user_id_has_access_by_link(0, "/test/test-article"));
    ///
    ///assert!(has_access);
    ///```
    async fn user_id_has_access_by_link(&self, id: usize, link: &str) -> bool;

    ///Check if user has been verified (currently via mail; in the future
    ///probably also via Google/Github Login)
    ///
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::models::RegisterUser;
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///};
    ///
    ///let rt = Runtime::new().unwrap();
    ///let _ = rt.block_on(db.create_user(new_user));
    ///
    ///let is_verified = rt.block_on(db.user_id_is_verified(0));
    ///assert!(!is_verified);
    ///```
    async fn user_id_is_verified(&self, id: usize) -> bool;

    ///Verifies the user
    ///
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::models::RegisterUser;
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///};
    ///
    ///let rt = Runtime::new().unwrap();
    ///let _ = rt.block_on(db.create_user(new_user));
    ///
    ///let _ = rt.block_on(db.confirm_email_for_user_id(0));
    ///
    ///let is_verified = rt.block_on(db.user_id_is_verified(0));
    ///assert!(is_verified);
    ///```
    async fn confirm_email_for_user_id(&self, id: usize) -> Result<(), ()>;

    ///Delete a user by providing their `user_id`.
    ///
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::models::RegisterUser;
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///};
    ///
    ///let rt = Runtime::new().unwrap();
    ///let _ = rt.block_on(db.create_user(new_user));
    ///
    ///let user_option = rt.block_on(db.get_user_by_id(0));
    ///assert!(user_option.is_some());
    ///
    ///let _ = rt.block_on(db.delete_user_by_id(0));
    ///let user_option = rt.block_on(db.get_user_by_id(0));
    ///assert!(user_option.is_none());
    ///```
    async fn delete_user_by_id(&self, id: usize) -> Result<(), ()>;

    ///If `user_id` exists, returns all `PaywallArticle`s that the corresponding
    ///user has access to. If user doesn't exist, returns `None`
    ///
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::models::{RegisterUser, LoginUser, PaywallArticle};
    ///use rust_server::price::Price;
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::security::NonHashing;
    ///
    ///let db: InMemoryDb<NonHashing> = InMemoryDb::new("123".to_string());
    ///let new_user = RegisterUser {
    ///     email: "test@test.com".to_string(),
    ///     username: "test".to_string(),
    ///     password: "insecure password".to_string(),
    ///     password_repeat: "insecure password".to_string()
    ///};
    ///
    ///let rt = Runtime::new().unwrap();
    ///let _ = rt.block_on(db.create_user(new_user));
    ///
    ///let articles = rt.block_on(db.get_paywall_articles_for_user_id(0)).unwrap();
    ///assert_eq!(articles.len(), 0);
    ///
    ///let article = PaywallArticle::new(
    ///     "test identifier".to_string(),
    ///     "/test/test-article".to_string(),
    ///     "test title".to_string(),
    ///     Price::from_currency_string(10, "USD").unwrap()
    ///);
    ///
    ///let _ = rt.block_on(db.add_accessible_article_to_id(0, article));
    ///let articles = rt.block_on(db.get_paywall_articles_for_user_id(0)).unwrap();
    ///assert_eq!(articles.len(), 1);
    ///```
    async fn get_paywall_articles_for_user_id(&self, id: usize) -> Option<Vec<PaywallArticle>>;
}
