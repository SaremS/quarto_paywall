use crate::models::AuthLevel;
use crate::paywall::{AuthLevelConditionalObject, SessionConditionalObject};

///Take some input and manipulate it in different ways.
///Each manipulation is then stored as a `SessionConditionalObject`.
///E.g. we take an HTML string that is to be paywalled and apply
///the respective paywall transformations that should be served at
///each auth level
pub trait SessionConditionalManipulation<S: Clone, T: Clone + Send + Sync, U: SessionConditionalObject<T>> {
    fn manipulate_object(&self, input: S) -> U;
}

pub struct AuthLevelManipulatorByFn<S: Clone, T: Send + Sync> {
    auth_and_funs: Vec<(AuthLevel, fn(S) -> T)>,
}

impl<S: Clone, T: Clone + Send + Sync> SessionConditionalManipulation<S, T, AuthLevelConditionalObject<T>>
    for AuthLevelManipulatorByFn<S, T>
{
    ///Create an `AuthLevelConditionalObject<T>` by performing different manipulations
    ///on an object of type `S` which ultimately return an object of type `T`.
    ///E.g. start with an input HTML as `&str` and return a manipulated page as a `String`.
    ///Each output might be a variant (e.g. paywalled VS. non-paywalled) of the original HTML
    ///document
    ///```
    ///use tokio::runtime::Runtime;
    ///use rust_server::paywall::{
    ///     SessionConditionalObject,
    ///     AuthLevelConditionalObject, 
    ///     AuthLevelManipulatorByFn,
    ///     SessionConditionalManipulation
    /// };
    ///use rust_server::models::{AuthLevel, SessionStatus};
    ///
    ///let input = "test";
    ///
    ///let manipulation: Vec<(AuthLevel, fn(&str)->String)> = vec![
    ///     (AuthLevel::NoAuth, |x| x.to_string() + "1"), 
    ///     (AuthLevel::UserConfirmed, |x| x.to_string() + "2")];
    ///
    ///let manipulator = AuthLevelManipulatorByFn::new(manipulation);
    ///
    ///let output = manipulator.manipulate_object(input);
    ///
    ///let session_status_noauth = SessionStatus{ user_id: None, auth_level: AuthLevel::NoAuth,
    ///username: None};
    ///let session_status_unconf = SessionStatus{ user_id: None, auth_level: AuthLevel::UserUnconfirmed, username: None};
    ///let session_status_conf = SessionStatus{ user_id: None, auth_level: AuthLevel::UserConfirmed, username: None};
    ///let session_status_admin = SessionStatus{ user_id: None, auth_level: AuthLevel::AdminAuth, username: None};
    ///
    ///let rt = Runtime::new().unwrap();
    ///
    ///let noauth = rt.block_on(output.get(&session_status_noauth));
    ///let unconf = rt.block_on(output.get(&session_status_unconf));
    ///let conf = rt.block_on(output.get(&session_status_conf));
    ///let admin = rt.block_on(output.get(&session_status_admin));
    ///
    ///assert_eq!(noauth, "test1");
    ///assert_eq!(unconf, "test1");
    ///assert_eq!(conf, "test2");
    ///assert_eq!(admin, "test2");
    ///```
    fn manipulate_object(&self, input: S) -> AuthLevelConditionalObject<T> {
        let mut items = Vec::new();

        //TODO: Possible without `.clone()`?
        for (auth_level, fun) in self.auth_and_funs.clone() {
            let content = fun(input.clone());
            items.push((auth_level, content));
        }

        return AuthLevelConditionalObject::new(items);
    }
}

impl<S: Clone, T: Clone + Send + Sync> AuthLevelManipulatorByFn<S, T> {
    ///Merely checks for increasing levels of AuthLevels and then
    ///creates the object
    pub fn new(auth_and_funs: Vec<(AuthLevel, fn(S) -> T)>) -> AuthLevelManipulatorByFn<S, T> {
        assert!(auth_and_funs.windows(2).all(|item| item[0].0 < item[1].0));
        return AuthLevelManipulatorByFn { auth_and_funs };
    }

    ///Presume that there is only a single transformation at the lowest
    ///AuthLevel (= AuthLevel::NoAuth)
    pub fn new_with_single_level(target_fun: fn(S) -> T) -> AuthLevelManipulatorByFn<S, T> {
        let auth_and_funs = vec![(AuthLevel::NoAuth, target_fun)];
        return AuthLevelManipulatorByFn { auth_and_funs };
    }
}
