use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{info, error};

pub struct SmtpCredentials<'a> {
    mail_address: &'a str,
    sender_name: &'a str,
    smtp_host: &'a str,
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

pub struct MailToSend<'a> {
    recipient_mail: &'a str,
    subject: &'a str,
    body: &'a str,
}

impl<'a> MailToSend<'a> {
    pub fn new(recipient_mail: &'a str, subject: &'a str, body: &'a str) -> MailToSend<'a> {
        return MailToSend {
            recipient_mail,
            subject,
            body,
        };
    }
}

pub async fn send_email<'a>(credentials: SmtpCredentials<'a>, mail: MailToSend<'a>) {
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
