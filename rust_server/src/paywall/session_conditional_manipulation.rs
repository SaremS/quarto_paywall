use crate::models::AuthLevel;
use crate::paywall::{AuthLevelConditionalObject, SessionConditionalObject};

///Take some input and manipulate it in different ways.
///Each manipulation is then stored as a `SessionConditionalObject`.
///E.g. we take an HTML string that is to be paywalled and apply
///the respective paywall transformations that should be served at
///each auth level
pub trait SessionConditionalManipulation<S, T: Copy + Send + Sync, U: SessionConditionalObject<T>> {
    fn manipulate_object(&self, input: &S) -> U;
}

///Create an `AuthLevelConditionalObject<T>` by performing different manipulations
///on an object of type `S` which ultimately return an object of type `T`.
///E.g. start with an input HTML as `&str` and return a manipulated page as a `String`.
///Each output might be a variant (e.g. paywalled VS. non-paywalled) of the original HTML
///document
struct AuthLevelManipulatorByFn<S, T: Copy + Send + Sync> {
    auth_and_funs: Vec<(AuthLevel, fn(&S) -> T)>,
}

impl<S, T: Copy + Send + Sync> SessionConditionalManipulation<S, T, AuthLevelConditionalObject<T>>
    for AuthLevelManipulatorByFn<S, T>
{
    fn manipulate_object(&self, input: &S) -> AuthLevelConditionalObject<T> {
        let mut items = Vec::new();

        for (auth_level, fun) in self.auth_and_funs.clone() {
            let content = fun(input);
            items.push((auth_level, content));
        }

        return AuthLevelConditionalObject::new(items);
    }
}

impl<S, T: Copy + Send + Sync> AuthLevelManipulatorByFn<S, T> {
    ///Merely checks for increasing levels of AuthLevels and then
    ///creates the object
    pub fn new(auth_and_funs: Vec<(AuthLevel, fn(&S) -> T)>) -> AuthLevelManipulatorByFn<S, T> {
        assert!(auth_and_funs.windows(2).all(|item| item[0].0 < item[1].0));
        return AuthLevelManipulatorByFn { auth_and_funs };
    }

    ///Presume that there is only a single transformation at the lowest
    ///AuthLevel (= AuthLevel::NoAuth)
    pub fn new_with_single_level(target_fun: fn(&S) -> T) -> AuthLevelManipulatorByFn<S, T> {
        let auth_and_funs = vec![(AuthLevel::NoAuth, target_fun)];
        return AuthLevelManipulatorByFn { auth_and_funs };
    }
}
