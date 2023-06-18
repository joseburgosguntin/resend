// #![cfg_attr(feature = "nightly", feature(once_cell))]

use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

use thiserror::Error;

pub mod api_keys;
pub mod domains;
pub mod emails;

/// https://resend.com/docs/api-reference/introduction
pub struct Resend {
    pub api_keys: api_keys::ApiKeys,
    pub domains: domains::Domains,
    pub emails: emails::Emails,
}

/// https://resend.com/docs/api-reference/errors
#[derive(Error, Debug)]
pub enum Error {
    /// https://resend.com/docs/api-reference/errors
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("You have already called set_api_key")]
    OnceCellWasAlreadySet,
    #[error("You have to call set_api_key")]
    OnceCellWasNotSet,
}

/// https://resend.com/docs/api-reference/errors
pub type Result<T> = std::result::Result<T, Error>;

impl Resend {
    pub fn new(api_key: &'static str) -> Result<Resend> {
        let client = std::sync::Arc::new({
            let mut header_map = HeaderMap::new();
            header_map.insert(
                AUTHORIZATION,
                format!("Bearer {}", api_key).try_into().unwrap(),
            );
            header_map.insert(CONTENT_TYPE, "application/json".try_into().unwrap());
            reqwest::Client::builder()
                .default_headers(header_map)
                .build()
                .unwrap()
        });
        Ok(Resend {
            api_keys: api_keys::ApiKeys {
                client: client.clone(),
            },
            domains: domains::Domains {
                client: client.clone(),
            },
            emails: emails::Emails { client },
        })
    }
}

#[cfg(feature = "global")]
use once_cell::sync::OnceCell;

#[cfg(feature = "global")]
pub(crate) static CLIENT: OnceCell<reqwest::Client> = OnceCell::new();

#[cfg(feature = "global")]
pub fn set_api_key(api_key: &'static str) -> Result<()> {
    CLIENT
        .set({
            let mut header_map = HeaderMap::new();
            header_map.insert(
                AUTHORIZATION,
                format!("Bearer {}", api_key).try_into().unwrap(),
            );
            header_map.insert(CONTENT_TYPE, "application/json".try_into().unwrap());
            reqwest::Client::builder()
                .default_headers(header_map)
                .build()
                .unwrap()
        })
        .map_err(|_| Error::OnceCellWasAlreadySet)
}

#[cfg(test)]
mod tests {
    // #[cfg(feature = "common")]
    use super::*;

    #[tokio::test]
    async fn get_email() -> Result<()> {
        let client = Resend::new("re_123")?;
        let res = client.emails.get("thing").await;

        assert!(res.is_ok());
        Ok(())
    }
    #[cfg(feature = "global")]
    #[tokio::test]
    async fn get_email() -> Result<()> {
        set_api_key("re_123")?;
        let res = emails::get("thing").await;

        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn send_email() -> Result<()> {
        let client = Resend::new("re_123")?;
        let req = emails::SendEmailRequest {
            from: "",
            html: "",
            text: "",
            subject: "",
            reply_to: "",
            to: &[""],
            cc: &[""],
            bcc: &[""],
            tags: &[emails::Tag {
                name: "",
                value: "",
            }],
            attachments: &[emails::Attachment {
                path: "",
                content: "",
                file_name: "",
            }],
        };
        let res = client.emails.send(req).await;

        assert!(res.is_ok());
        Ok(())
    }

    #[cfg(feature = "global")]
    #[tokio::test]
    async fn send_email() -> Result<()> {
        set_api_key("re_123")?;
        let req = emails::SendEmailRequest {
            from: "",
            html: "",
            text: "",
            subject: "",
            reply_to: "",
            to: &[""],
            cc: &[""],
            bcc: &[""],
            tags: &[emails::Tag {
                name: "",
                value: "",
            }],
            attachments: &[emails::Attachment {
                path: "",
                content: "",
                file_name: "",
            }],
        };
        let res = emails::send(req).await;

        assert!(res.is_ok());
        Ok(())
    }
}
