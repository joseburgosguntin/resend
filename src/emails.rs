use crate::Result;
use serde::{Deserialize, Serialize};

/// https://resend.com/docs/api-reference/emails
pub struct Emails {
    pub client: std::sync::Arc<reqwest::Client>,
}

/// https://resend.com/docs/api-reference/emails/send-email#body-parameters
#[derive(Serialize)]
pub struct SendEmailRequest<'a> {
    pub from: &'a str,
    pub to: &'a [&'a str],
    pub subject: &'a str,
    pub bcc: &'a [&'a str],
    pub cc: &'a [&'a str],
    pub reply_to: &'a str,
    pub html: &'a str,
    pub text: &'a str,
    pub tags: &'a [Tag<'a>],
    pub attachments: &'a [Attachment<'a>],
}

/// https://resend.com/docs/api-reference/emails/send-email#body-parameters
#[derive(Serialize)]
pub struct Tag<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

/// https://resend.com/docs/api-reference/emails/send-email#body-parameters
#[derive(Serialize)]
pub struct Attachment<'a> {
    pub content: &'a str,
    pub file_name: &'a str,
    pub path: &'a str,
}

/// https://resend.com/docs/api-reference/emails/send-email
#[derive(Deserialize)]
pub struct SendEmailResponse {
    pub id: Box<str>,
}

/// https://resend.com/docs/api-reference/emails/retrieve-email
#[derive(Deserialize)]
pub struct Email {
    pub id: Box<str>,
    pub object: Box<str>,
    pub to: Box<[Box<str>]>,
    pub from: Box<str>,
    pub created_at: Box<str>,
    pub subject: Box<str>,
    pub html: Box<str>,
    pub text: Box<str>,
    pub bcc: Box<[Box<str>]>,
    pub cc: Box<[Box<str>]>,
    pub reply_to: Box<[Box<str>]>,
    pub last_event: Box<[Box<str>]>,
}

impl Emails {
    /// https://resend.com/docs/api-reference/emails/send-email
    pub async fn send<'a>(&self, params: SendEmailRequest<'a>) -> Result<SendEmailResponse> {
        Ok(self
            .client
            .post(concat!("https://api.resend.com/", "emails"))
            .json(&params)
            .send()
            .await?
            .json()
            .await?)
    }

    /// https://resend.com/docs/api-reference/emails/retrieve-email
    pub async fn get<'a>(&self, email_id: &'a str) -> Result<Email> {
        Ok(self
            .client
            .get(concat!("https://api.resend.com/", "emails/").to_string() + email_id)
            .send()
            .await
            .map_err(|e| e)?
            .json()
            .await?)
    }
}

#[cfg(feature = "global")]
use crate::CLIENT;

/// https://resend.com/docs/api-reference/emails/send-email
#[cfg(feature = "global")]
pub async fn send<'a>(params: SendEmailRequest<'a>) -> Result<SendEmailResponse> {
    Ok(CLIENT
        .get()
        .ok_or(crate::Error::OnceCellWasNotSet)?
        .post(concat!("https://api.resend.com/", "emails"))
        .json(&params)
        .send()
        .await?
        .json()
        .await?)
}

/// https://resend.com/docs/api-reference/emails/retrieve-email
#[cfg(feature = "global")]
pub async fn get<'a>(email_id: &'a str) -> Result<Email> {
    Ok(CLIENT
        .get()
        .ok_or(crate::Error::OnceCellWasNotSet)?
        .post(concat!("https://api.resend.com/", "emails/").to_string() + email_id)
        .send()
        .await?
        .json()
        .await?)
}
