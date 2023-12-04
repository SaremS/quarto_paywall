extern crate rust_server;
use crate::common::{MockStripeClient, MockEmailClient};

mod common;

#[test]
fn simple_userflow_test() {
    //register - verify - purchase - delete

    use tokio::runtime::Runtime;

    use regex::Regex;
    use rust_server::database::{Database, InMemoryDb};
    use rust_server::models::{PaywallArticle, PurchaseIntent, PurchaseReference, RegisterUser};
    use rust_server::price::Price;
    use rust_server::purchase::PurchaseHandler;
    use rust_server::security::NonHashing;
    use rust_server::user_communication::UserCommunicator;
    use rust_server::utils::ResultOrInfo;

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

    let mock_email_client = MockEmailClient {
        expected_recipient: create_user.email.clone(),
        expected_subject: "Please confirm your email address".to_string(),
        expected_body: "TEST".to_string()
    };
    //TODO: Better handling of body - need to somehow convert the token in the body back to the
    //corresponding userid

    let verification_handler = UserCommunicator::new(
        "test_mail_secret_key".to_string(),
        "test_deletion_secret_key".to_string(),
        "https://test.com".to_string(),
        Box::new(mock_email_client)
    );

    let user_id = create_user.user_id;
    let recipient = create_user.email;

    let _ = rt
        .block_on(verification_handler.send_registration_verification_email(&user_id, &recipient));


    let _ = rt.block_on(db.confirm_email_for_user_id(user_id));
    let is_verified = rt.block_on(db.user_id_is_verified(user_id));
    assert!(is_verified);

    let purchase_intent = PurchaseIntent {
        purchase_target: "/mock-target".to_string(),
    };

    let article = PaywallArticle::new(
        "test_identifier".to_string(),
        "test_link".to_string(),
        "test_title".to_string(),
        Price::from_currency_string(100, "USD").unwrap(),
    );

    let purchase_reference = PurchaseReference {
        user_id,
        article: article.clone(),
    };

    let payload = "test_payload";
    let signature = "test_signature";

    let stripe_client = Box::new(MockStripeClient {
        expected_purchase_reference: purchase_reference.clone(),
        expected_domainpath: "test.com/mock-target",
        expected_payload: payload,
        expected_signature: signature,
    });

    let handler = PurchaseHandler::new("test.com", stripe_client);

    let checkout_result = rt
        .block_on(handler.stripe_checkout(&user_id, &purchase_intent, &article))
        .unwrap();
    
    assert_eq!(&checkout_result, "Output");

    let webhook_result = rt.block_on(handler.stripe_webhook_to_purchase_reference(payload, signature));
    if let ResultOrInfo::Ok(reference) = webhook_result {
        assert_eq!(reference, purchase_reference);
    } else {
        panic!();
    }

    let _ = rt.block_on(db.add_accessible_article_to_id(0, article));

    assert!(rt.block_on(db.user_id_has_access_by_link(0, "test_link")));
}
