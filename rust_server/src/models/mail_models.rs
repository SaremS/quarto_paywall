use lettre::transport::smtp::authentication::Credentials;
use serde::{Serialize, Deserialize};

pub struct MailEnvVars<'a> {
    pub mail_secret_key: &'a str,
    pub smtp_mail_address: &'a str,
    pub domain_url: &'a str,
    pub smtp_host: &'a str,
    pub smtp_sender_name: &'a str,
    pub smtp_username: &'a str,
    pub smtp_password: &'a str,
}

pub struct SmtpCredentials<'a> {
    mail_address: &'a str,
    sender_name: &'a str,
    pub smtp_host: &'a str,
    username: &'a str,
    password: &'a str,
}

impl<'a> SmtpCredentials<'a> {
    pub fn new(
        mail_address: &'a str,
        sender_name: &'a str,
        smtp_host: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> SmtpCredentials<'a> {
        return SmtpCredentials {
            mail_address,
            sender_name,
            smtp_host,
            username,
            password,
        };
    }

    pub fn get_full_sender(&self) -> String {
        return self.sender_name.to_owned() + "<" + self.mail_address + ">";
    }

    pub fn get_lettre_smtp_credentials(&self) -> Credentials {
        return Credentials::new(self.username.to_owned(), self.password.to_owned());
    }
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

#[derive(Serialize, Deserialize)]
pub struct EmailConfirmation {
    pub user_id: usize,
    pub exp: usize,
}
