use log::error;
use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SignupError {
    #[error("Passwords don't match!")]
    PasswordsDontMatchError,
    #[error("Email `{0}` already exists!")]
    EmailExistsError(String),
    #[error("The following errors were found while validating your inputs: `{0}`")]
    RegistrationFieldsError(String),
}
