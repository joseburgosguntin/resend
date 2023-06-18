use serde::Deserialize;
use serde::Serialize;

use crate::Result;

/// https://resend.com/docs/api-reference/domains
pub struct Domains {
    pub client: std::sync::Arc<reqwest::Client>,
}

/// https://resend.com/docs/api-reference/domains/create-domain#body-parameters
#[derive(Serialize)]
pub struct CreateDomainRequest<'a> {
    pub name: &'a str,
    pub region: &'a Region,
}

/// https://resend.com/docs/api-reference/domains/create-domain#body-parameters
#[derive(Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "us-east-1")]
    UsEast1,
    #[serde(rename = "eu-west-1")]
    EuWest1,
    #[serde(rename = "sa-east-1")]
    SaEast1,
}

/// https://resend.com/docs/api-reference/domains/create-domain
#[derive(Deserialize)]
pub struct CreateDomainResponse {
    pub id: Box<str>,
    pub name: Box<str>,
    #[serde(rename = "createdAt")]
    pub created_at: Box<str>,
}

/// https://resend.com/docs/api-reference/domains/list-domains
#[derive(Deserialize)]
pub struct ListDomainResponse {
    pub data: Box<[Domain]>,
}

/// https://resend.com/docs/api-reference/domains/list-domains
#[derive(Deserialize)]
pub struct Domain {
    pub id: Box<str>,
    pub object: Box<str>,
    pub name: Box<str>,
    pub create_at: Box<str>,
    pub status: Box<str>,
    pub region: Region,
}

impl Domains {
    /// https://resend.com/docs/api-reference/domains/create-domain
    pub async fn create<'a>(
        &self,
        params: CreateDomainRequest<'a>,
    ) -> Result<CreateDomainResponse> {
        Ok(self
            .client
            .post(concat!("https://api.resend.com/", "domains"))
            .json(&params)
            .send()
            .await?
            .json()
            .await?)
    }

    /// https://resend.com/docs/api-reference/domains/get-domain
    pub async fn get<'a>(&self, domain_id: &'a str) -> Result<Domain> {
        Ok(self
            .client
            .delete(concat!("https://api.resend.com/", "domains").to_string() + domain_id)
            .send()
            .await?
            .json()
            .await?)
    }

    /// https://resend.com/docs/api-reference/domains/verify-domain
    pub async fn verify<'a>(&self, domain_id: &'a str) -> Result<()> {
        _ = self
            .client
            .post(concat!("https://api.resend.com/", "domains").to_string() + domain_id)
            .send()
            .await?;
        Ok(())
    }
    /// https://resend.com/docs/api-reference/domains/list-domains
    pub async fn list(&self) -> Result<ListDomainResponse> {
        Ok(self
            .client
            .get(concat!("https://api.resend.com/", "domains"))
            .send()
            .await?
            .json()
            .await?)
    }

    /// https://resend.com/docs/api-reference/domains/delete-domain
    pub async fn delete<'a>(&self, domain_id: &'a str) -> Result<()> {
        _ = self
            .client
            .delete(concat!("https://api.resend.com/", "domains").to_string() + domain_id)
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(feature = "global")]
use crate::CLIENT;

/// https://resend.com/docs/api-reference/domains/create-domain
#[cfg(feature = "global")]
pub async fn create<'a>(params: CreateDomainRequest<'a>) -> Result<CreateDomainResponse> {
    Ok(CLIENT
        .post(concat!("https://api.resend.com/", "domains"))
        .json(&params)
        .send()
        .await?
        .json()
        .await?)
}

/// https://resend.com/docs/api-reference/domains/get-domain
#[cfg(feature = "global")]
pub async fn get<'a>(domain_id: &'a str) -> Result<Domain> {
    Ok(CLIENT
        .delete(concat!("https://api.resend.com/", "domains").to_string() + domain_id)
        .send()
        .await?
        .json()
        .await?)
}

/// https://resend.com/docs/api-reference/domains/verify-domain
#[cfg(feature = "global")]
pub async fn verify<'a>(domain_id: &'a str) -> Result<()> {
    _ = CLIENT
        .post(concat!("https://api.resend.com/", "domains").to_string() + domain_id)
        .send()
        .await?;
    Ok(())
}
/// https://resend.com/docs/api-reference/domains/list-domains
#[cfg(feature = "global")]
pub async fn list() -> Result<ListDomainResponse> {
    Ok(CLIENT
        .get(concat!("https://api.resend.com/", "domains"))
        .send()
        .await?
        .json()
        .await?)
}

/// https://resend.com/docs/api-reference/domains/delete-domain
#[cfg(feature = "global")]
pub async fn delete<'a>(domain_id: &'a str) -> Result<()> {
    _ = CLIENT
        .delete(concat!("https://api.resend.com/", "domains").to_string() + domain_id)
        .send()
        .await?;
    Ok(())
}
