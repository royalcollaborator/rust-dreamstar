use dotenv::dotenv;

// dotenv init
pub fn init() {
    dotenv().ok();
}
pub const RECAPTCHA_SITE_KEY: &str = dotenv!("RECAPTCHA_SITE_KEY");
pub const SERVER_URL: &str = dotenv!("SERVER_URL");
pub const HOST_URL: &str = dotenv!("HOST_URL");
