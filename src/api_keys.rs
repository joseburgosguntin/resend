use crate::Result;
use serde::{Deserialize, Serialize};

/// https://resend.com/docs/api-reference/api-keys
pub struct ApiKeys {
    pub client: std::sync::Arc<reqwest::Client>,
}

/// https://resend.com/docs/api-reference/api-keys/create-api-key#body-parameters
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    FullAccess,
    SendingAccess,
}

/// https://resend.com/docs/api-reference/api-keys/create-api-key#body-parameters
#[derive(Serialize)]
pub struct CreateApiKeyRequest<'a> {
    pub name: &'a str,
    pub permission: Option<Permission>,
    pub domain_id: Option<&'a str>,
}

/// https://resend.com/docs/api-reference/api-keys/create-api-key
#[derive(Deserialize)]
pub struct CreateApiKeyResponse {
    pub id: Box<str>,
    pub token: Box<str>,
}

/// https://resend.com/docs/api-reference/api-keys/list-api-keys
#[derive(Debug, Deserialize)]
pub struct ListApiKeysResponse {
    pub data: Box<[ApiKey]>,
}

/// https://resend.com/docs/api-reference/api-keys/list-api-keys
#[derive(Debug, Deserialize)]
pub struct ApiKey {
    pub id: Box<str>,
    pub name: Box<str>,
    pub created_at: Box<str>,
}

impl ApiKeys {
    /// https://resend.com/docs/api-reference/api-keys/create-api-key
    pub async fn create<'a>(
        &self,
        params: CreateApiKeyRequest<'a>,
    ) -> Result<CreateApiKeyResponse> {
        Ok(self
            .client
            .post(concat!("https://api.resend.com/", "api-keys"))
            .json(&params)
            .send()
            .await?
            .json()
            .await?)
    }

    /// https://resend.com/docs/api-reference/api-keys/list-api-keys
    pub async fn list<'a>(&self) -> Result<ListApiKeysResponse> {
        Ok(self
            .client
            .get(concat!("https://api.resend.com/", "api-keys"))
            .send()
            .await?
            .json()
            .await?)
    }

    /// https://resend.com/docs/api-reference/api-keys/delete-api-key
    pub async fn delete<'a>(&self) -> Result<()> {
        _ = self
            .client
            .delete(concat!("https://api.resend.com/", "api-keys"))
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(feature = "global")]
use crate::Result;
#[cfg(feature = "global")]
use crate::CLIENT;

/// https://resend.com/docs/api-reference/api-keys/create-api-key
#[cfg(feature = "global")]
pub async fn create<'a>(params: CreateApiKeyRequest<'a>) -> Result<CreateApiKeyResponse> {
    Ok(CLIENT
        .get()
        .ok_or(crate::Error::OnceCellWasNotSet)?
        .post(concat!("https://api.resend.com/", "api-keys"))
        .json(&params)
        .send()
        .await?
        .json()
        .await?)
}

/// https://resend.com/docs/api-reference/api-keys/list-api-keys
#[cfg(feature = "global")]
pub async fn list<'a>() -> Result<ListApiKeysResponse> {
    Ok(CLIENT
        .get()
        .ok_or(crate::Error::OnceCellWasNotSet)?
        .get(concat!("https://api.resend.com/", "api-keys"))
        .send()
        .await?
        .json()
        .await?)
}

/// https://resend.com/docs/api-reference/api-keys/delete-api-key
#[cfg(feature = "global")]
pub async fn delete<'a>() -> Result<()> {
    _ = CLIENT
        .get()
        .ok_or(crate::Error::OnceCellWasNotSet)?
        .delete(concat!("https://api.resend.com/", "api-keys"))
        .send()
        .await?;
    Ok(())
}
