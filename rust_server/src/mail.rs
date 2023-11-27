use chrono::{Duration, Utc};
use log::{error, info};

use async_trait::async_trait;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lettre::message::header::ContentType;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::envvars::EnvVarLoader;

#[async_trait]
pub trait VerifyAndDeleteUser: Send + Sync {
    async fn send_registration_verification(&self, user_id: &usize, recipient: &str);
    async fn handle_registration_verification(
        &self,
        token: &str,
    ) -> Result<usize, VerificationError>;

    async fn send_deletion_verification(&self, user_id: &usize, recipient: &str);
    async fn handle_deletion_verification(&self, token: &str) -> Result<usize, VerificationError>;
}

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("There's a problem with your confirmation link - it might have expired. Please request another one.")]
    TokenError,
}

pub struct EmailDevice {
    mail_secret_key: String,
    deletion_secret_key: String,
    smtp_mail_address: String,
    domain_url: String,
    smtp_host: String,
    smtp_sender_name: String,
    smtp_username: String,
    smtp_password: String,
}

#[async_trait]
impl VerifyAndDeleteUser for EmailDevice {
    async fn send_registration_verification(&self, user_id: &usize, recipient: &str) {
        let token = self.make_verification_token(user_id, self.mail_secret_key.clone(), Duration::days(1)).await;
        let confirm_url = "".to_owned() + &self.domain_url + "/confirm-user?token=" + &token;

        let subject = "Please confirm your email address";
        let body: String = "Thanks for registering at ".to_string()
            + &self.domain_url
            + "! As a last step, please follow this confirmation link: \n"
            + &confirm_url;

        let verification_mail = EmailToSend::new(recipient, subject, body);

        self.send_email(verification_mail).await;
    }

    async fn handle_registration_verification(
        &self,
        token: &str,
    ) -> Result<usize, VerificationError> {
        let key = self.mail_secret_key.clone();

        let verification_result = self.decode_verification_token(token, &key).await;

        return verification_result.map(|x| x.user_id);
    }

    async fn send_deletion_verification(&self, user_id: &usize, recipient: &str) {
        let token =
            self.make_verification_token(user_id, self.deletion_secret_key.clone(), Duration::minutes(15)).await;

        let confirm_url = "".to_owned() + &self.domain_url + "/delete-user?token=" + &token;

        let subject = "Please confirm your request for account deletion";
        let body: String = "Thanks for registering at ".to_string()
            + &self.domain_url
            + "! As a last step, please follow this confirmation link: \n"
            + &confirm_url;

        let deletion_mail = EmailToSend::new(recipient, subject, body);

        self.send_email(deletion_mail).await;
    }

    async fn handle_deletion_verification(
        &self,
        token: &str,
    ) -> Result<usize, VerificationError> {
        let key = self.deletion_secret_key.clone();

        let verification_result = self.decode_verification_token(token, &key).await;

        return verification_result.map(|x| x.user_id);
    }
}

impl EmailDevice {
    pub fn new_from_envvars(loader: &EnvVarLoader) -> EmailDevice {
        return EmailDevice {
            mail_secret_key: loader.get_mail_secret_key(),
            deletion_secret_key: loader.get_deletion_secret_key(),
            smtp_mail_address: loader.get_smtp_mail_address(),
            domain_url: loader.get_domain_url(),
            smtp_host: loader.get_smtp_host(),
            smtp_sender_name: loader.get_smtp_sender_name(),
            smtp_username: loader.get_smtp_username(),
            smtp_password: loader.get_smtp_password()
        };
    }

    async fn send_email<'a>(&self, mail: EmailToSend<'a>) {
        let email = Message::builder()
            .from(self.get_full_sender().await.parse().unwrap())
            .to(mail.recipient_mail.parse().unwrap())
            .subject(mail.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(mail.body))
            .unwrap();

        let smtp_creds = self.get_lettre_smtp_credentials().await;

        let mailer = SmtpTransport::relay(&self.smtp_host)
            .unwrap()
            .credentials(smtp_creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => info!("Email sent successfully!"),
            Err(e) => error!("Could not send email: {e:?}"),
        }
    }

    async fn get_lettre_smtp_credentials(&self) -> Credentials {
        return Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());
    }

    async fn get_full_sender(&self) -> String {
        return self.smtp_sender_name.clone() + "<" + &self.smtp_mail_address + ">";
    }

    async fn decode_verification_token(
        &self,
        token: &str,
        key: &str
    ) -> Result<EmailVerification, VerificationError> {
        let decoded = decode::<EmailVerification>(
            token,
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::default(),
        );

        match decoded {
            Ok(verified) => {return Ok(verified.claims);},
            Err(_) => {return Err(VerificationError::TokenError);}
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

#[derive(Serialize, Deserialize)]
struct EmailVerification {
    pub user_id: usize,
    pub exp: usize,
}

pub struct EmailToSend<'a> {
    pub recipient_mail: &'a str,
    pub subject: &'a str,
    pub body: String,
}

impl<'a> EmailToSend<'a> {
    pub fn new(recipient_mail: &'a str, subject: &'a str, body: String) -> EmailToSend<'a> {
        return EmailToSend {
            recipient_mail,
            subject,
            body,
        };
    }
}
