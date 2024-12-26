use crate::shared::TestEnv;
use actix_web::web::Data;
use actix_web::{test, App};
use base64::prelude::*;
use oclus_server::api::route::define_routes;
use rand::Rng;
use serde_json::json;
use sha2::Sha256;
use srp::client::SrpClient;
use srp::groups::G_2048;

mod shared;

#[actix_web::test]
async fn test_pre_registration() {
    let test_env = TestEnv::new().await;
    let mut app = test::init_service(
        App::new()
            .configure(define_routes)
            .app_data(Data::new(test_env.services.clone()))
            .app_data(Data::new(test_env.config.clone()))
            .app_data(Data::new(test_env)),
    )
    .await;

    let req_body = json!({
        "email": "test@test.com"
    });

    let resp = test::TestRequest::post()
        .uri("/user/register/pre")
        .set_json(&req_body)
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_registration() {
    let test_env = TestEnv::new().await;
    let mut app = test::init_service(
        App::new()
            .configure(define_routes)
            .app_data(Data::new(test_env.services.clone()))
            .app_data(Data::new(test_env.config.clone())),
    )
    .await;

    let pre_registration_id: i64 = sqlx::query_scalar(
        r#"INSERT INTO pre_registrations (email, otp) VALUES ($1, $2) RETURNING id"#,
    )
    .bind("test@test.com".to_string())
    .bind("123456".to_string())
    .fetch_one(&test_env.db)
    .await
    .unwrap();

    let client = SrpClient::<Sha256>::new(&G_2048);

    let mut salt = [0u8; 32];
    rand::thread_rng().fill(&mut salt);

    let verifier = client.compute_verifier("test".as_bytes(), "password".as_bytes(), &salt);

    let salt_b64 = BASE64_STANDARD.encode(&salt);
    let verifier_b64 = BASE64_STANDARD.encode(&verifier);

    let req_body = json!({
        "pre_registration_id": pre_registration_id,
        "otp": "123456",
        "username": "test",
        "srp_verifier": verifier_b64,
        "srp_salt": salt_b64
    });

    let resp = test::TestRequest::post()
        .uri("/user/register")
        .set_json(&req_body)
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());
}
