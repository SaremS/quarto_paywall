use crate::memoryserving::{AuthLevelConditionalObject, SessionConditionalObject};


///Take some input and manipulate it in different ways. 
///Each manipulation is then stored as a `SessionConditionalObject`.
///E.g. we take an HTML string that is to be paywalled and apply
///the respective paywall transformations that should be served at
///each auth level
pub trait SessionConditionalManipulation<T: Copy + Send + Sync> {
    fn manipulate_object(input: T) -> dyn SessionConditionalObject<T>;
}

pub trait AuthLevelConditionalManipulation<T: Copy + Send + Sync>: SessionConditionalManipulation<T> {
    fn manipulate_object(input: T) -> AuthLevelConditionalObject<T>;
}
