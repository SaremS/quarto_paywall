use async_trait::async_trait;

use crate::models::{AuthLevel, SessionStatus};

///Get stored object based on some session status property
///
///Primarily to serve distinct objects based on auth-level,
///e.g. paywalled content VS. paywall banner; `T` would
///then be made concrete as `String` or `&str`; other options
///are, however, possible as well
///
///Should be straightforward to extend e.g. to locale
#[async_trait]
pub trait SessionConditionalObject<T: Clone + Send + Sync>: Send + Sync {
    async fn get(&self, session_status: &SessionStatus) -> T;

    ///For upstream performance improvements - e.g. 304 Not Modified responses
    async fn get_hash(&self, session_status: &SessionStatus) -> String;
}

///Serve content based on user auth level
pub struct AuthLevelConditionalObject<T: Clone + Send + Sync> {
    //store as three distinct vecs to ease access
    auth_levels: Vec<AuthLevel>,
    contents: Vec<T>,
    hashes: Vec<String>
}

#[async_trait]
impl<T: Clone + Send + Sync> SessionConditionalObject<T> for AuthLevelConditionalObject<T> {
    ///Serve content based on user auth level
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::paywall::{AuthLevelConditionalObject, SessionConditionalObject};
    ///use rust_server::models::{SessionStatus, AuthLevel};
    ///use rust_server::security::xor_hash;
    ///
    ///let target_items = vec![(AuthLevel::NoAuth, "no auth"), (AuthLevel::UserConfirmed, "confirmed")];
    ///let conditional = AuthLevelConditionalObject::new(target_items, xor_hash);
    ///
    ///let session_status_noauth = SessionStatus{ user_id: None, auth_level: AuthLevel::NoAuth,
    ///username: None};
    ///let session_status_unconf = SessionStatus{ user_id: None, auth_level: AuthLevel::UserUnconfirmed, username: None};
    ///let session_status_conf = SessionStatus{ user_id: None, auth_level: AuthLevel::UserConfirmed, username: None};
    ///let session_status_admin = SessionStatus{ user_id: None, auth_level: AuthLevel::AdminAuth, username: None};
    ///
    ///let rt = Runtime::new().unwrap();
    ///let noauth = rt.block_on(conditional.get(&session_status_noauth));
    ///let unconf = rt.block_on(conditional.get(&session_status_unconf));
    ///let conf = rt.block_on(conditional.get(&session_status_conf));
    ///let admin = rt.block_on(conditional.get(&session_status_admin));
    ///
    ///assert_eq!(noauth, "no auth");
    ///assert_eq!(unconf, "no auth");
    ///assert_eq!(conf, "confirmed");
    ///assert_eq!(admin, "confirmed");
    ///```
    async fn get(&self, session_status: &SessionStatus) -> T {
        let auth_level = session_status.auth_level;
        return self.get_with_auth_level(&auth_level);
    }

    async fn get_hash(&self, session_status: &SessionStatus) -> String {
        let auth_level = session_status.auth_level;
        return self.get_hash_with_auth_level(&auth_level);
    }
}

impl<T: Clone + Send + Sync> AuthLevelConditionalObject<T> {
    ///Serve files based on auth level. `assert!`s that the `AuthLevel`s in
    ///`items` are in **strictly** increasing order - panics if not.
    pub fn new(items: Vec<(AuthLevel, T)>, hash_fun: fn(T) -> String) -> AuthLevelConditionalObject<T> {
        //require items as tuples but store as three separate vectors to avoid messing
        //up which auth level belongs to which content
        assert!(items.windows(2).all(|item| item[0].0 < item[1].0));

        let mut auth_levels = Vec::new();
        let mut contents = Vec::new();
        let mut hashes = Vec::new();

        for (auth_level, content) in items {
            auth_levels.push(auth_level);
            contents.push(content.clone());
            hashes.push(hash_fun(content));
        }

        return AuthLevelConditionalObject {
            auth_levels,
            contents,
            hashes
        };
    }

    pub fn new_with_single_level(content: T, hash_fun: fn(T) -> String) -> AuthLevelConditionalObject<T> {
        let auth_levels = vec![AuthLevel::NoAuth];
        let contents = vec![content.clone()];
        let hashes = vec![hash_fun(content)];

        return AuthLevelConditionalObject {
            auth_levels,
            contents,
            hashes
        };
    }

    fn get_with_auth_level(&self, auth_level: &AuthLevel) -> T {
        let idx = self.get_auth_level_index(auth_level);
        return self.contents[idx].clone();
    }

    fn get_hash_with_auth_level(&self, auth_level: &AuthLevel) -> String {
        let idx = self.get_auth_level_index(auth_level);
        return self.hashes[idx].clone();
    }

    fn get_auth_level_index(&self, auth_level: &AuthLevel) -> usize {
        let lesser_levels: usize = self
            .auth_levels
            .iter()
            .map(|level| (auth_level >= level) as usize)
            .sum();

        let target_index = lesser_levels - 1; //subtract 1 due to 0-based indexing

        return target_index;
    }
}
