//! Email sending utilities to interface with SES.

use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use tracing::{error, info};

/// Subject and the body of the email are always in UTF-8.
pub const CHARSET: &str = "UTF-8";
/// All emails are sent from this address.
pub const FROM: &str = "Bite-sized learning <max@bitesized.info>";

/// Sends a plain text email using SES.
/// All errors are logged inside the function.
pub async fn send_text_email(to: &str, subject: &str, body: &str) {
    info!("Sending {subject} email to: {to}, body: {body}");

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_sesv2::Client::new(&config);

    // prepare TO field
    let dest = Destination::builder().to_addresses(to).build();

    // prepare SUBJECT field
    let subject_content = match Content::builder().data(subject).charset(CHARSET).build() {
        Ok(v) => v,
        Err(e) => {
            error!("Error building subject content: {:?}", e);
            return;
        }
    };

    // prepare BODY field
    let body_content = match Content::builder().data(body).charset(CHARSET).build() {
        Ok(v) => v,
        Err(e) => {
            error!("Error building body content: {:?}", e);
            return;
        }
    };

    // put everything together
    let email_content = EmailContent::builder()
        .simple(
            Message::builder()
                .subject(subject_content)
                .body(Body::builder().text(body_content).build())
                .build(),
        )
        .build();

    // send out
    match client
        .send_email()
        .from_email_address(FROM)
        .destination(dest)
        .content(email_content)
        .send()
        .await
    {
        Ok(_) => info!("Email sent"),
        Err(e) => error!("Error sending email: {e}"),
    };
}
