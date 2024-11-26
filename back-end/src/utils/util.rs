use crate::config::{jwt_secret, token_expire};
use bcrypt::hash;
use chrono::{Duration, Utc};
use json5::{from_str, Error};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use mongodb::bson::oid::ObjectId;
use rand::seq::SliceRandom;
use rand::Rng;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum DecodeJwtHelper {
    Ok(TokenData<Claims>),
    Err,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize,
}

/**
 * Encoded JWT token
 * parameters {id : ObjectId, role : i32, secret : str, expiration : i64}
 * return : (String, Err)
 */
pub fn encode_jwt(id: ObjectId) -> Result<String, jsonwebtoken::errors::Error> {
    let token_expire_time = convert_str_to_i32(token_expire().as_str());
    let expiration = (Utc::now() + Duration::hours(token_expire_time.into())).timestamp() as usize;
    let my_claims = Claims {
        user_id: id.to_string(),
        exp: expiration as usize,
    };
    match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(jwt_secret().as_ref()),
    ) {
        Ok(token) => Ok(token),
        Err(e) => Err(e),
    }
}

/**
 * Decoded JWT token
 * parameters {token : String}
 * return :  DecodeJwtHelper
 */
pub fn decode_jwt(token: String) -> DecodeJwtHelper {
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret().as_ref()),
        &Validation::default(),
    );
    match token {
        Ok(token_string) => DecodeJwtHelper::Ok(token_string),
        Err(_) => DecodeJwtHelper::Err,
    }
}

/**
 * Hash password
 */
pub fn hash_text(text: String, cost: u32) -> Result<String, Status> {
    return match hash(text, cost) {
        Ok(hash_text) => Ok(hash_text),
        Err(_) => Err(Status::BadGateway),
    };
}

/**
 * Generate random number for otp
 */
pub fn generate_otp() -> String {
    let mut rng = rand::thread_rng();
    let otp: String = (0..5)
        .map(|_| rng.gen_range(0..10)) // Generate a random number between 0 and 9
        .map(|num| std::char::from_digit(num, 10).unwrap()) // Convert the number to a char
        .collect(); // Collect chars into a String

    otp
}
/**
 * Generate random string for live battle
 */
pub fn live_battle_code() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .chars()
        .collect();
    let code: String = (0..9)
        .map(|_| {
            if rng.gen_bool(0.5) {
                // Randomly choose a character from the chars vector
                *chars.choose(&mut rng).unwrap()
            } else {
                std::char::from_digit(rng.gen_range(0..10), 10).unwrap()
            }
        })
        .collect();

    code
}

/**
 * Convert str into i32
 */
pub fn convert_str_to_i32(s: &str) -> i32 {
    match s.parse::<i32>() {
        Ok(val) => val,
        Err(_) => 0,
    }
}
