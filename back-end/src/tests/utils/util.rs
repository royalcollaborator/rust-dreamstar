use mongodb::bson::oid::ObjectId;

use crate::config::*;
use crate::utils::util::*;
use bcrypt::{verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

#[test]
fn test_encode_jwt() {
    let id = ObjectId::new();

    let token = encode_jwt(id).expect("Failed to encode JWT");

    assert!(!token.is_empty(), "Token should not be empty");
}

#[test]
fn test_decode_jwt() {
    let token_expire_time = convert_str_to_i32(token_expire().as_str());
    let id = ObjectId::new();
    let expiration = (Utc::now() + Duration::hours(token_expire_time.into())).timestamp() as usize;
    let my_claims = Claims {
        user_id: id.to_hex(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(jwt_secret().as_ref()),
    )
    .expect("Failed to encode JWT");

    let decoded = decode_jwt(token.clone());

    match decoded {
        DecodeJwtHelper::Ok(data) => {
            assert_eq!(data.claims.user_id, id.to_hex(), "User IDs should match");
        }
        DecodeJwtHelper::Err => panic!("Failed to decode JWT"),
    }
}

#[test]
fn test_hash_text() {
    let password = String::from("my-secret-password");
    let cost = DEFAULT_COST;

    let hashed_password = hash_text(password.clone(), cost).expect("Failed to hash password");

    assert!(
        verify(&password, &hashed_password).unwrap(),
        "Passwords should match"
    );
}

#[test]
fn test_generate_otp() {
    let otp = generate_otp();

    assert_eq!(otp.len(), 5, "OTP should be 5 characters long");
    assert!(
        otp.chars().all(|c| c.is_digit(10)),
        "OTP should only contain digits"
    );
}
