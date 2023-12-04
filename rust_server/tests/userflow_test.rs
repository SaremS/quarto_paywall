extern crate rust_server;

use async_trait::async_trait;

use rust_server::user_communication::EmailSender;

struct Recorder {
    recipient: Option<String>,
    subject: Option<String>,
    body: Option<String>
}

impl Recorder {
    pub fn new(self) -> Recorder {
        return Recorder {recipient: None, subject: None, body: None};
    }
    pub fn set(&mut self, recipient: &str, subject: &str, body: &str) {
        self.recipient = Some(recipient.to_string());
        self.subject = Some(subject.to_string());
        self.body = Some(body.to_string());
    }
}

struct MockEmailSender<'a> {
    recorder: &'a mut Recorder
}

#[async_trait]
impl<'a> EmailSender for MockEmailSender<'a> {
    async fn send(&self, recipient: &str, subject: &str, body: &str) -> Result<(),()> {
        self.recorder.set(recipient, subject, body);
        return Ok(());
    }
}


#[test]
fn simple_userflow_test() {
    //register - verify - purchase - delete



    use tokio::runtime::Runtime;

    use rust_server::database::{Database, InMemoryDb};
    use rust_server::models::RegisterUser;
    use rust_server::security::ScryptHashing;
    use rust_server::user_communication::{EmailSender, VerificationHandler};

    let rt = Runtime::new().unwrap();

    let db: InMemoryDb<ScryptHashing> = InMemoryDb::new("test_jwt".to_string());
    let register_user = RegisterUser {
        email: "test@test.com".to_string(),
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
        password_repeat: "testpassword".to_string(),
    };

    let create_user_result = rt.block_on(db.create_user(register_user));

    assert!(create_user_result
        .is_ok_and(|x| x.user_id == 0 && x.email == "test@test.com" && x.username == "testuser"));


    let email_device = VerificationHandler::new(
        "test_mail_secret_key".to_string(),
        "test_deletion_secret_key".to_string(),
        "https://test.com".to_string(),
    );
}
