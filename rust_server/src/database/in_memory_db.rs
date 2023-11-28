use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tokio::sync::Mutex;
use validator::Validate;

use crate::database::Database;
use crate::errors::AuthenticationError;
use crate::errors::SignupError;
use crate::models::{LoginUser, PaywallArticle, RegisterUser, User, UserCreated, UserLoggedIn};
use crate::security::HashingAlgorithm;

pub struct InMemoryDb<T> where T: HashingAlgorithm {
    ///Simple pseudo-DB for testing
    ///
    ///=> Do not use in production
    db: Arc<Mutex<HashMap<String, User>>>, //email -> User
    id_index: Arc<Mutex<HashMap<usize, String>>>, //id -> email
    username_index: Arc<Mutex<HashMap<String, String>>>, //username->email
    jwt_secret: String,
    hashing_algorithm: T,
}

impl<T: HashingAlgorithm> InMemoryDb<T> {
    pub fn new(jwt_secret: String, hashing_algorithm: T) -> InMemoryDb<T> {
        return InMemoryDb {
            db: Arc::new(Mutex::new(HashMap::new())),
            id_index: Arc::new(Mutex::new(HashMap::new())),
            username_index: Arc::new(Mutex::new(HashMap::new())),
            jwt_secret,
            hashing_algorithm,
        };
    }
}

#[async_trait]
impl<T: HashingAlgorithm> Database for InMemoryDb<T> {
    ///Create new user and directly provide login JWT after successful signup
    ///```
    ///use tokio::runtime::Runtime;
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::models::RegisterUser;
    ///use rust_server::security::NonHashing;
    ///
    ///let db = InMemoryDb::new("123".to_string(), NonHashing{});
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
    async fn create_user(&self, user: RegisterUser) -> Result<UserCreated, SignupError> {
        match user.validate() {
            Ok(_) => (),
            Err(e) => return Err(SignupError::RegistrationFieldsError(e.to_string())),
        }

        let mut local_db = self.db.lock().await;
        let mut local_id_index = self.id_index.lock().await;
        let mut local_username_index = self.username_index.lock().await;

        if local_db.contains_key(&user.email) {
            return Err(SignupError::EmailExistsError(user.email));
        }

        if local_username_index.contains_key(&user.username) {
            return Err(SignupError::UsernameExistsError(user.username));
        }

        let new_id = local_db.len().clone();

        let created_user = User {
            id: new_id,
            time_registered: Utc::now().timestamp() as usize,
            email: user.email,
            username: user.username,
            password: T::get_hash(&user.password),
            is_verified: false,
            role: "user".to_string(),
            accessible_articles: HashSet::new(),
        };

        match created_user.validate() {
            Ok(_) => (),
            Err(e) => return Err(SignupError::RegistrationFieldsError(e.to_string())),
        }

        local_db.insert(created_user.email.clone(), created_user.clone());
        local_id_index.insert(new_id, created_user.email.clone());
        local_username_index.insert(created_user.username.clone(), created_user.email.clone());

        let token = crate::security::get_jwt_for_user(&created_user, &self.jwt_secret);

        let user_created = UserCreated {
            user_id: new_id,
            email: created_user.email,
            username: created_user.username.clone(),
            jwt: token,
        };

        return Ok(user_created);
    }

    ///Create user with admin credentials
    async fn create_admin(&self, user: RegisterUser) -> Result<UserCreated, SignupError> {
        match user.validate() {
            Ok(_) => (),
            Err(e) => return Err(SignupError::RegistrationFieldsError(e.to_string())),
        }

        let mut local_db = self.db.lock().await;
        let mut local_id_index = self.id_index.lock().await;

        if local_db.contains_key(&user.email) {
            return Err(SignupError::EmailExistsError(user.email));
        }

        let new_id = local_db.len().clone();

        let created_user = User {
            id: new_id,
            time_registered: Utc::now().timestamp() as usize,
            email: user.email,
            username: user.username,
            password: T::get_hash(&user.password),
            is_verified: false,
            role: "admin".to_string(),
            accessible_articles: HashSet::new(),
        };

        match created_user.validate() {
            Ok(_) => (),
            Err(e) => return Err(SignupError::RegistrationFieldsError(e.to_string())),
        }

        local_db.insert(created_user.email.clone(), created_user.clone());
        local_id_index.insert(new_id, created_user.email.clone());

        let token = crate::security::get_jwt_for_user(&created_user, &self.jwt_secret);

        let user_created = UserCreated {
            user_id: new_id,
            email: created_user.email,
            username: created_user.username.clone(),
            jwt: token,
        };

        return Ok(user_created);
    }

    ///Match provided credentials against stored credentials and
    ///grant access token if they are matching.
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::models::{RegisterUser, LoginUser};
    ///use rust_server::database::{Database, InMemoryDb};
    ///use rust_server::security::NonHashing;
    ///
    ///let db = InMemoryDb::new("123".to_string(), NonHashing{});
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
    async fn login(&self, login_user: LoginUser) -> Result<UserLoggedIn, AuthenticationError> {
        let cur_user_db = self.db.lock().await;

        let user = match cur_user_db.get(&login_user.email) {
            Some(k) => k,
            None => {
                return Err(AuthenticationError::UserNotFoundError);
            }
        };

        if !T::verify_hash(&login_user.password, &user.password)
        {
            return Err(AuthenticationError::InvalidCredentialsError);
        }

        let token = crate::security::get_jwt_for_user(user, &self.jwt_secret);
        let user_logged_in = UserLoggedIn {
            username: user.username.clone(),
            jwt: token,
        };

        return Ok(user_logged_in);
    }

    async fn get_user_by_id(&self, id: usize) -> Option<User> {
        let local_db = self.db.lock().await;
        let local_id_index = self.id_index.lock().await;

        let email_option = local_id_index.get(&id);

        match email_option {
            Some(email) => return Some(local_db.get(email).unwrap().clone()),
            None => return None,
        }
    }

    async fn add_accessible_article_to_id(
        &self,
        id: usize,
        article: PaywallArticle,
    ) -> Result<(), ()> {
        let mut local_db = self.db.lock().await;
        let local_id_index = self.id_index.lock().await;

        let email_option = local_id_index.get(&id);

        match email_option {
            Some(email) => {
                if let Some(user) = local_db.get_mut(email) {
                    user.accessible_articles.insert(article);
                    return Ok(());
                } else {
                    return Err(());
                }
            }
            None => return Err(()),
        }
    }

    async fn confirm_email_for_user_id(&self, id: usize) -> Result<(), ()> {
        let mut local_db = self.db.lock().await;
        let local_id_index = self.id_index.lock().await;

        let email_option = local_id_index.get(&id);

        match email_option {
            Some(email) => {
                if let Some(user) = local_db.get_mut(email) {
                    user.is_verified = true;
                    return Ok(());
                } else {
                    return Err(());
                }
            }
            None => return Err(()),
        }
    }

    async fn user_id_has_access_by_link(&self, id: usize, link: &str) -> bool {
        if let Some(user) = self.get_user_by_id(id).await {
            return user
                .accessible_articles
                .into_iter()
                .any(|x| x.link_matches(link));
        } else {
            return false;
        }
    }

    async fn user_id_is_verified(&self, id: usize) -> bool {
        if let Some(user) = self.get_user_by_id(id).await {
            return user.is_verified;
        } else {
            return false;
        }
    }

    async fn delete_user_by_id(&self, id: usize) -> Result<(), ()> {
        let mut local_db = self.db.lock().await;
        let mut local_id_index = self.id_index.lock().await;
        let mut local_username_index = self.username_index.lock().await;

        let email_option = local_id_index.get(&id);

        match email_option {
            Some(email) => {
                //find and remove username index
                let username = &local_db.get(email).unwrap().username;
                local_username_index.remove(username);

                //find and remove user from db
                local_db.remove(email);

                //find and remove id index
                local_id_index.remove(&id);
                return Ok(());
            }
            None => {
                return Err(());
            }
        }
    }

    async fn get_paywall_articles_for_user_id(&self, id: usize) -> Option<Vec<PaywallArticle>> {
        if let Some(user) = self.get_user_by_id(id).await {
            return Some(user.accessible_articles.into_iter().collect());
        } else {
            return None;
        }
    }
}
