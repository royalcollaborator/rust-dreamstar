use std::env;

// A helper function to get environment variables at runtime
fn get_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} is not set", key))
}

pub fn jwt_secret() -> String {
    get_env_var("JWT_SECRET")
}

pub fn db_url() -> String {
    get_env_var("DB_URL")
}

pub fn db_name() -> String {
    get_env_var("DB_NAME")
}

pub fn smtp_email() -> String {
    get_env_var("SMTP_EMAIL")
}

pub fn smtp_password() -> String {
    get_env_var("SMTP_PASSWORD")
}

pub fn recaptcha_key() -> String {
    get_env_var("RECAPTCHA_KEY")
}

pub fn token_expire() -> String {
    get_env_var("TOKEN_EXPIRE")
}

pub fn email_expire() -> String {
    get_env_var("EMAIL_EXPIRE")
}

pub fn google_client_id() -> String {
    get_env_var("GOOGLE_CLIENT_ID")
}

pub fn google_client_secret() -> String {
    get_env_var("GOOGLE_CLIENT_SECRET")
}

pub fn google_auth_url() -> String {
    get_env_var("GOOGLE_AUTH_URL")
}

pub fn google_token_url() -> String {
    get_env_var("GOOGLE_TOKEN_URL")
}

pub fn google_redirect_url() -> String {
    get_env_var("GOOGLE_REDIRECT_URL")
}

pub fn instagram_client_id() -> String {
    get_env_var("INSTAGRAM_CLIENT_ID")
}

pub fn instagram_client_secret() -> String {
    get_env_var("INSTAGRAM_CLIENT_SECRET")
}

pub fn instagram_redirect_url() -> String {
    get_env_var("INSTAGRAM_REDIRECT_URL")
}

pub fn test_recaptcha_key() -> String {
    get_env_var("TEST_RECAPTCHA_KEY")
}

pub fn google_img_bucket() -> String {
    get_env_var("GOOGLE_IMG_BUCKET")
}

pub fn google_vid_bucket() -> String {
    get_env_var("GOOGLE_VID_BUCKET")
}

pub fn google_key_json() -> String {
    get_env_var("GOOGLE_KEY_JSON")
}

pub fn is_test() -> String {
    get_env_var("TEST")
}
