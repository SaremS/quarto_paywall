extern crate rust_server;

#[test]
fn simple_userflow_test() {
    //register - verify - purchase - delete

    use tokio::runtime::Runtime;

    use regex::Regex;
    use rust_server::database::{Database, InMemoryDb};
    use rust_server::models::RegisterUser;
    use rust_server::security::NonHashing;
    use rust_server::user_communication::VerificationHandler;

    let rt = Runtime::new().unwrap();

    let db: InMemoryDb<NonHashing> = InMemoryDb::new("test_jwt".to_string());
    let register_user = RegisterUser {
        email: "test@test.com".to_string(),
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
        password_repeat: "testpassword".to_string(),
    };

    let create_user = rt.block_on(db.create_user(register_user)).unwrap();

    assert_eq!(create_user.user_id, 0);
    assert_eq!(create_user.email, "test@test.com");
    assert_eq!(create_user.username, "testuser");

    let verification_handler = VerificationHandler::new(
        "test_mail_secret_key".to_string(),
        "test_deletion_secret_key".to_string(),
        "https://test.com".to_string(),
    );

    let user_id = create_user.user_id;
    let recipient = create_user.email;

    let email = rt
        .block_on(verification_handler.make_registration_verification_email(&user_id, &recipient));

    assert_eq!(email.recipient, recipient);

    let body = email.body;
    let token = Regex::new(r"token=([^&]*)")
        .unwrap()
        .captures(&body)
        .and_then(|caps| caps.get(1).map(|match_| match_.as_str().to_string()))
        .unwrap();

    let extracted_user_id = rt
        .block_on(verification_handler.handle_registration_verification(&token))
        .unwrap();
    assert_eq!(extracted_user_id, user_id);

    let _ = rt.block_on(db.confirm_email_for_user_id(extracted_user_id));
    let is_verified = rt.block_on(db.user_id_is_verified(extracted_user_id));
    assert!(is_verified);


    
}
