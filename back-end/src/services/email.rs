use crate::config::{smtp_email, smtp_password};
use lettre::message::header::ContentType;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

/**
 * Send verification mail to user's email
 */

pub fn send_email(email: String, subject: String, body: String) -> Result<bool, String> {
    let credential = Credentials::new(smtp_email().to_string(), smtp_password().to_string());
    let mailer_handler = SmtpTransport::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(credential)
        .build();

    let email_send = match Message::builder()
        .from(smtp_email().parse().unwrap())
        .to(email.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body)
    {
        Ok(res) => res,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    match mailer_handler.send(&email_send) {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}
