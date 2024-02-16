//! Implementation of the Authentication API.

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::models;

const GET_TOKEN_ENDPOINT: &str = "/oauth/token";

/// Implementation of the authentication API.
#[derive(Clone)]
pub struct AuthenticationApi(Arc<Inner>);

struct Inner {
    /// The tenant's domain.
    domain: Url,
    /// Application's Client ID.
    client_id: String,
    /// Application's Client Secret.
    client_secret: Option<String>,
    /// HTTP client.
    client: Client,
}

impl std::fmt::Debug for AuthenticationApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthenticationApi")
            .field("domain", &self.0.domain)
            .field("client_id", &self.0.client_id)
            .finish()
    }
}

impl AuthenticationApi {
    /// Create a new istance of the authentication API.
    pub fn new(domain: &str, client_id: &str) -> Result<Self> {
        let inner = Inner {
            domain: Url::parse(domain).context("Auth0 domain is not a valid url")?,
            client_id: client_id.to_owned(),
            client_secret: None,
            client: Client::new(),
        };
        Ok(Self(Arc::new(inner)))
    }

    /// Create a new istance of the authentication API.
    pub fn with_client_secret(domain: &str, client_id: &str, client_secret: &str) -> Result<Self> {
        let inner = Inner {
            domain: Url::parse(domain).context("Auth0 domain is not a valid url")?,
            client_id: client_id.to_owned(),
            client_secret: Some(client_secret.to_owned()),
            client: Client::new(),
        };
        Ok(Self(Arc::new(inner)))
    }

    /// Send a POST request to the given endpoint.
    async fn http_post<B, T>(&self, endpoint: &str, json: &B) -> Result<T>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        self.0
            .client
            .post(self.0.domain.join(endpoint)?)
            .json(json)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Directly request an access token by using the [client's credentials].
    ///
    /// [client's credentials]: https://auth0.com/docs/api/authentication#client-credentials-flow
    pub fn get_token<T: Into<String>>(&self, audience: T) -> Result<ClientCredentialsFlow<'_>> {
        Ok(ClientCredentialsFlow {
            api: self,
            grant_type: "client_credentials",
            client_id: &self.0.client_id,
            client_secret: self
                .0
                .client_secret
                .as_ref()
                .ok_or_else(|| anyhow!("Missing client_secret"))?,
            audience: audience.into(),
        })
    }
}

/// Get an access token by using the client's credentials.
#[derive(Debug, Serialize)]
pub struct ClientCredentialsFlow<'a> {
    #[serde(skip)]
    api: &'a AuthenticationApi,
    grant_type: &'static str,
    client_id: &'a str,
    client_secret: &'a str,
    audience: String,
}

impl<'a> ClientCredentialsFlow<'a> {
    /// Send the API request.
    pub async fn send(&self) -> Result<models::AccessToken> {
        self.api.http_post(GET_TOKEN_ENDPOINT, self).await
    }
}
