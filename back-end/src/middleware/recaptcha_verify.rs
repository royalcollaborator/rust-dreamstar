use crate::models::error_response::ErrorRes;
use crate::utils::recaptcha::is_valid_recaptcha;
use crate::config::is_test;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Recaptcha {
    pub recaptcha_result: bool,
}

/**
 * Passport function
 * Every request mush go though this function
 * If recaptcha verify success, can proceed
 */
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Recaptcha {
    type Error = ErrorRes;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let recaptcha_key = match request.headers().get_one("RecaptchaKey") {
            Some(key) => key,
            None => {
                return Outcome::Error((
                    Status::Forbidden,
                    ErrorRes {
                        cause: "Please resend if you are not robot".to_string(),
                    },
                ));
            }
        };
        let recaptcha_name = match request.headers().get_one("recaptchaName") {
            Some(name) => name,
            None => {
                return Outcome::Error((
                    Status::Forbidden,
                    ErrorRes {
                        cause: "Please resend if you are not robot".to_string(),
                    },
                ));
            }
        };
        if is_test() == "true".to_string() {
            return Outcome::Success(Recaptcha {
                recaptcha_result: true,
            })
        }

        let result = is_valid_recaptcha(recaptcha_key.to_string(), recaptcha_name.to_string()).await;

        if result {
            Outcome::Success(Recaptcha {
                recaptcha_result: true,
            })
        } else {
            Outcome::Error((
                Status::Forbidden,
                ErrorRes {
                    cause: "Please resend if you are not robot".to_string(),
                },
            ))
        }
    }
}
