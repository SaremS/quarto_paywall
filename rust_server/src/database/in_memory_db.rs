use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use validator::Validate;

use crate::database::Database;
use crate::errors::AuthenticationError;
use crate::errors::SignupError;
use crate::models::{LoginUser, RegisterUser, User, UserCreated, UserLoggedIn};

pub struct InMemoryDb {
    db: Arc<Mutex<HashMap<String, User>>>,
    id_index: Arc<Mutex<HashMap<usize, String>>>,
    jwt_secret: String,
}

impl InMemoryDb {
    pub fn new(jwt_secret: String) -> InMemoryDb {
        return InMemoryDb {
            db: Arc::new(Mutex::new(HashMap::new())),
            id_index: Arc::new(Mutex::new(HashMap::new())),
            jwt_secret,
        };
    }
}

#[async_trait]
impl Database for InMemoryDb {
    async fn create_user(&self, user: RegisterUser) -> Result<UserCreated, SignupError> {
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
            email: user.email,
            username: user.username,
            password: crate::security::get_hash(&user.password),
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

        let token = crate::security::get_jwt_for_user(&created_user, &self.jwt_secret);

        let user_created = UserCreated {
            user_id: new_id,
            email: created_user.email,
            username: created_user.username.clone(),
            jwt: token,
        };

        return Ok(user_created);
    }

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
            email: user.email,
            username: user.username,
            password: crate::security::get_hash(&user.password),
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

    async fn login(&self, login_user: LoginUser) -> Result<UserLoggedIn, AuthenticationError> {
        let cur_user_db = self.db.lock().await;

        let user = match cur_user_db.get(&login_user.email) {
            Some(k) => k,
            None => {
                return Err(AuthenticationError::UserNotFoundError);
            }
        };

        if !crate::security::verify_hash(&login_user.password, &user.password) {
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

    async fn add_accessible_article_to_id(&self, id: usize, article: String) -> Result<(), ()> {
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

    async fn user_id_has_article_access(&self, id: usize, article: String) -> bool {
        use log::debug;
        debug!("{:?}", article);
        if let Some(user) = self.get_user_by_id(id).await {
            return user.accessible_articles.contains(&article);
        } else {
            return false;
        }
    }

    async fn delete_user_by_id(&self, id: usize) -> Result<(),()> {
        let mut local_db = self.db.lock().await;
        let mut local_id_index = self.id_index.lock().await;

        let email_option = local_id_index.get(&id);
        
        match email_option {
            Some(email) => {
                local_db.remove(email);
                local_id_index.remove(&id);
                return Ok(());
            },
            None => {return Err(());}
        }
    }
}
