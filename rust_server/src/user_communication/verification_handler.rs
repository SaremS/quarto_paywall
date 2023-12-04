use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::user_communication::EmailMessage;

pub struct VerificationHandler {
    mail_secret_key: String,
    deletion_secret_key: String,
    domain_url: String,
}

impl VerificationHandler {
    pub fn new(
        mail_secret_key: String,
        deletion_secret_key: String,
        domain_url: String,
    ) -> VerificationHandler {
        return VerificationHandler {
            mail_secret_key,
            deletion_secret_key,
            domain_url,
        };
    }

    pub async fn make_registration_verification_email(
        &self,
        user_id: &usize,
        recipient: &str,
    ) -> EmailMessage {
        let token = self
            .make_verification_token(user_id, self.mail_secret_key.clone(), Duration::days(1))
            .await;
        let confirm_url = "".to_owned() + &self.domain_url + "/confirm-user?token=" + &token;

        let subject = "Please confirm your email address";
        let body: String = "Thanks for registering at ".to_string()
            + &self.domain_url
            + "! As a last step, please follow this confirmation link: \n"
            + &confirm_url;

        let message = EmailMessage {
            recipient: recipient.to_string(), 
            subject: subject.to_string(), 
            body};

        return message;
    }

    pub async fn handle_registration_verification(
        &self,
        token: &str,
    ) -> Result<usize, VerificationError> {
        let key = self.mail_secret_key.clone();

        let verification_result = self.decode_verification_token(token, &key).await;

        return verification_result.map(|x| x.user_id);
    }

    pub async fn make_deletion_verification_email(
        &self,
        user_id: &usize,
        recipient: &str,
    ) -> EmailMessage {
        let token = self
            .make_verification_token(
                user_id,
                self.deletion_secret_key.clone(),
                Duration::minutes(15),
            )
            .await;

        let confirm_url = "".to_owned() + &self.domain_url + "/delete-user?token=" + &token;

        let subject = "Please confirm your request for account deletion";
        let body: String = "Thanks for registering at ".to_string()
            + &self.domain_url
            + "! As a last step, please follow this confirmation link: \n"
            + &confirm_url;

        let message = EmailMessage {
            recipient: recipient.to_string(), 
            subject: subject.to_string(), 
            body};

        return message;
    }

    pub async fn handle_deletion_verification(
        &self,
        token: &str,
    ) -> Result<usize, VerificationError> {
        let key = self.deletion_secret_key.clone();

        let verification_result = self.decode_verification_token(token, &key).await;

        return verification_result.map(|x| x.user_id);
    }

    async fn decode_verification_token(
        &self,
        token: &str,
        key: &str,
    ) -> Result<EmailVerification, VerificationError> {
        let decoded = decode::<EmailVerification>(
            token,
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::default(),
        );

        match decoded {
            Ok(verified) => {
                return Ok(verified.claims);
            }
            Err(_) => {
                return Err(VerificationError::TokenError);
            }
        }
    }

    async fn make_verification_token(
        &self,
        user_id: &usize,
        key: String,
        duration_till_expiry: Duration,
    ) -> String {
        let exp = Utc::now()
            .checked_add_signed(duration_till_expiry)
            .expect("invalid timestamp")
            .timestamp();

        let confirmation = EmailVerification {
            user_id: user_id.clone(),
            exp: exp as usize,
        };

        let token = match encode(
            &Header::default(),
            &confirmation,
            &EncodingKey::from_secret(key.as_bytes()),
        ) {
            Ok(t) => t,
            Err(_) => panic!(),
        };

        return token;
    }
}

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("There's a problem with your confirmation link - it might have expired. Please request another one.")]
    TokenError,
}

#[derive(Serialize, Deserialize)]
struct EmailVerification {
    pub user_id: usize,
    pub exp: usize,
}
