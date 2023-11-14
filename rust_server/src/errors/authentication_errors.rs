use log::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("User not found")]
    UserNotFoundError,
    #[error("Invalid password")]
    InvalidCredentialsError,
    #[error("Invalid jwt token")]
    InvalidJWTTokenError,
    #[error("Jwt token creation error")]
    JWTTokenCreationError,
    #[error("Authorization header required")]
    AuthHeaderRequiredError,
    #[error("Invalid auth header")]
    InvalidAuthHeaderError,
    #[error("Not authorized")]
    NotAuthorizedError,
}
