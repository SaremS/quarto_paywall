use chrono::{Duration, Utc};
use log::{error, info};

use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};

use crate::models::{
    DeletionConfirmation, EmailConfirmation, EmailToSend, MailEnvVars, SmtpCredentials,
};

pub async fn send_deletion_mail<'a>(
    user_id: &usize,
    recipient: &str,
    mail_environment: &'a MailEnvVars<'a>,
) {
    let exp = Utc::now()
        .checked_add_signed(Duration::minutes(30))
        .expect("invalid timestamp")
        .timestamp();

    let confirmation = DeletionConfirmation {
        user_id: user_id.clone(),
        exp: exp as usize,
    };

    let token = match encode(
        &Header::default(),
        &confirmation,
        &EncodingKey::from_secret(mail_environment.deletion_secret_key.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    let deletion_mail = make_deletion_mail(recipient, mail_environment.domain_url, &token);
    let smtp_credentials = SmtpCredentials::new(
        mail_environment.smtp_mail_address,
        mail_environment.smtp_sender_name,
        mail_environment.smtp_host,
        mail_environment.smtp_username,
        mail_environment.smtp_password,
    );

    send_email(smtp_credentials, deletion_mail).await;
}

pub async fn send_confirmation_mail<'a>(
    user_id: &usize,
    recipient: &str,
    mail_environment: &'a MailEnvVars<'a>,
) {
    let exp = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("invalid timestamp")
        .timestamp();

    let confirmation = EmailConfirmation {
        user_id: user_id.clone(),
        exp: exp as usize,
    };

    let token = match encode(
        &Header::default(),
        &confirmation,
        &EncodingKey::from_secret(mail_environment.mail_secret_key.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    let confirmation_mail = make_confirmation_mail(recipient, mail_environment.domain_url, &token);
    let smtp_credentials = SmtpCredentials::new(
        mail_environment.smtp_mail_address,
        mail_environment.smtp_sender_name,
        mail_environment.smtp_host,
        mail_environment.smtp_username,
        mail_environment.smtp_password,
    );

    send_email(smtp_credentials, confirmation_mail).await;
}

fn make_confirmation_mail<'a>(
    recipient_mail: &'a str,
    domain_url: &str,
    token: &str,
) -> EmailToSend<'a> {
    let confirm_url = domain_url.to_string() + "/confirm-user?token=" + token;

    let subject = "Please confirm your email address";
    let body: String = "Thanks for registering at ".to_string()
        + domain_url
        + "! As a last step, please follow this confirmation link: \n"
        + &confirm_url;

    return EmailToSend::new(recipient_mail, subject, body);
}

fn make_deletion_mail<'a>(
    recipient_mail: &'a str,
    domain_url: &str,
    token: &str,
) -> EmailToSend<'a> {
    let confirm_url = domain_url.to_string() + "/delete-user?token=" + token;

    let subject = "Please confirm your request for account deletion";
    let body: String = "Thanks for registering at ".to_string()
        + domain_url
        + "! As a last step, please follow this confirmation link: \n"
        + &confirm_url;

    return EmailToSend::new(recipient_mail, subject, body);
}

pub async fn send_email<'a>(credentials: SmtpCredentials<'a>, mail: EmailToSend<'a>) {
    let email = Message::builder()
        .from(credentials.get_full_sender().parse().unwrap())
        .to(mail.recipient_mail.parse().unwrap())
        .subject(mail.subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(mail.body))
        .unwrap();

    let smtp_creds = credentials.get_lettre_smtp_credentials();

    let mailer = SmtpTransport::relay(&credentials.smtp_host)
        .unwrap()
        .credentials(smtp_creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => info!("Email sent successfully!"),
        Err(e) => error!("Could not send email: {e:?}"),
    }
}
