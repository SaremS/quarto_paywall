use std::collections::HashMap;
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
}

impl InMemoryDb {
    pub fn new() -> InMemoryDb {
        return InMemoryDb {
            db: Arc::new(Mutex::new(HashMap::new())),
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

        if local_db.contains_key(&user.email) {
            return Err(SignupError::EmailExistsError(user.email));
        }

        let created_user = User {
            email: user.email,
            username: user.username,
            password: crate::security::get_hashed_password(&user.password),
            is_verified: false,
            role: "user".to_string(),
            accessible_articles: Vec::new()
        };

        match created_user.validate() {
            Ok(_) => (),
            Err(e) => return Err(SignupError::RegistrationFieldsError(e.to_string())),
        }

        local_db.insert(created_user.email.clone(), created_user.clone());
        let token = crate::security::get_jwt_for_user((created_user).clone());
        
        let user_created = UserCreated {
            username: created_user.username.clone(),
            jwt: token
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

        if !crate::security::verify_password(&login_user.password, &user.password) {
            return Err(AuthenticationError::InvalidCredentialsError);
        }

        let token = crate::security::get_jwt_for_user((*user).clone());
        let user_logged_in = UserLoggedIn {
            username: user.username.clone(),
            jwt: token,
        };

        return Ok(user_logged_in);
    }
}
