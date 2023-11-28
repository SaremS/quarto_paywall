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
pub trait SessionConditionalObject<T: Copy + Send + Sync>: Send + Sync {
    async fn get(&self, session_status: &SessionStatus) -> T;
}

///Serve content based on user auth level
pub struct AuthLevelConditionalObject<T: Copy> {
    //store as two distinct object to ease access
    auth_levels: Vec<AuthLevel>,
    contents: Vec<T>,
}

#[async_trait]
impl<T: Copy + Send + Sync> SessionConditionalObject<T> for AuthLevelConditionalObject<T> {
    ///Serve content based on user auth level
    ///```
    ///use tokio::runtime::Runtime;
    ///
    ///use rust_server::memoryserving::{AuthLevelConditionalObject, SessionConditionalObject};
    ///use rust_server::models::{SessionStatus, AuthLevel};
    ///
    ///let target_items = vec![(AuthLevel::NoAuth, "no auth"), (AuthLevel::UserConfirmed, "confirmed")];
    ///let conditional = AuthLevelConditionalObject::new(target_items);
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
}

impl<T: Copy> AuthLevelConditionalObject<T> {
    ///Serve files based on auth level. `assert!`s that the `AuthLevel`s in
    ///`items` are in **strictly** increasing order - panics if not.
    pub fn new(items: Vec<(AuthLevel, T)>) -> AuthLevelConditionalObject<T> {
        //require items as tuples but store as two separate vectors to avoid messing
        //up which auth level belongs to which content
        assert!(items.windows(2).all(|item| item[0].0 < item[1].0));

        let mut auth_levels = Vec::new();
        let mut contents = Vec::new();

        for (auth_level, content) in items {
            auth_levels.push(auth_level);
            contents.push(content);
        }

        return AuthLevelConditionalObject {
            auth_levels,
            contents,
        };
    }

    pub fn new_with_single_level(content: T) -> AuthLevelConditionalObject<T> {
        let auth_levels = vec![AuthLevel::NoAuth];
        let contents = vec![content];

        return AuthLevelConditionalObject {
            auth_levels,
            contents,
        };
    }

    fn get_with_auth_level(&self, auth_level: &AuthLevel) -> T {
        let idx = self.get_auth_level_index(auth_level);
        return self.contents[idx];
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
