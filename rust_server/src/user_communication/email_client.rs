use async_trait::async_trait;
use lettre::message::header::ContentType;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

use crate::envvars::EnvVarLoader;

pub struct EmailMessage {
    pub recipient: String,
    pub subject: String,
    pub body: String,
}

#[async_trait]
pub trait EmailSender: Sync + Send {
    async fn send(&self, message: &EmailMessage) -> Result<(), ()>;
}

pub struct EmailClient {
    smtp_mail_address: String,
    smtp_host: String,
    smtp_sender_name: String,
    smtp_username: String,
    smtp_password: String,
}

#[async_trait]
impl EmailSender for EmailClient {
    async fn send(&self, message: &EmailMessage) -> Result<(), ()> {
        let email = Message::builder()
            .from(self.get_full_sender().await.parse().unwrap())
            .to(message.recipient.parse().unwrap())
            .subject(message.subject.clone())
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(message.body.clone()))
            .unwrap();

        let smtp_creds = self.get_lettre_smtp_credentials().await;

        let mailer = SmtpTransport::relay(&self.smtp_host)
            .unwrap()
            .credentials(smtp_creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => Err(()),
        }
    }
}

impl EmailClient {
    pub fn new(
        smtp_mail_address: String,
        smtp_host: String,
        smtp_sender_name: String,
        smtp_username: String,
        smtp_password: String,
    ) -> EmailClient {
        return EmailClient {
            smtp_mail_address,
            smtp_host,
            smtp_sender_name,
            smtp_username,
            smtp_password,
        };
    }

    pub fn new_from_envvars(loader: &EnvVarLoader) -> EmailClient {
        return EmailClient {
            smtp_mail_address: loader.get_smtp_mail_address(),
            smtp_host: loader.get_smtp_host(),
            smtp_sender_name: loader.get_smtp_sender_name(),
            smtp_username: loader.get_smtp_username(),
            smtp_password: loader.get_smtp_password(),
        };
    }

    async fn get_lettre_smtp_credentials(&self) -> Credentials {
        return Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());
    }

    async fn get_full_sender(&self) -> String {
        return self.smtp_sender_name.clone() + "<" + &self.smtp_mail_address + ">";
    }
}