extern crate rust_server;

#[test]
fn simple_userflow_test() {
    //register - verify - purchase - delete
    use tokio::runtime::Runtime;

    use rust_server::database::{Database, InMemoryDb};
    use rust_server::security::ScryptHashing;
    use rust_server::models::RegisterUser;
    use rust_server::user_communication::{EmailDevice, VerifyAndDeleteUser};

    let rt = Runtime::new().unwrap();

    let db: InMemoryDb<ScryptHashing> = InMemoryDb::new("test_jwt".to_string()); 
    let register_user = RegisterUser { 
        email: "test@test.com".to_string(),
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
        password_repeat: "testpassword".to_string()
    };

    let create_user_result = rt.block_on(db.create_user(register_user));

    assert!(create_user_result.is_ok_and(
            |x| x.user_id==0 && x.email=="test@test.com" && x.username=="testuser")
        );

    let email_device = EmailDevice::new(
                        "test_mail_secret_key".to_string(),
                        "test_deletion_secret_key".to_string(),
                        "test@smtp.test".to_string(),
                        "test-url.com".to_string(),
                        "test.smtp.host".to_string(),
                        "test_name".to_string(),
                        "test_username".to_string(),
                        "test_password".to_string());
}
