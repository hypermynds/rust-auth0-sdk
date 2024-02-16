use std::sync::Arc;

use anyhow::{Context, Result};
use reqwest::{
    header::{self, HeaderValue},
    Client, Method, Request, Url,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Clients, Users};

/// Implementation of the management API.
#[derive(Clone)]
pub struct ManagementApi(Arc<Inner>);

struct Inner {
    /// The tenant's domain.
    domain: Url,
    /// The token to authenticate the calls with.
    api_token: HeaderValue,
    /// Default HTTP client.
    client: Client,
}

impl std::fmt::Debug for ManagementApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManagementApi")
            .field("domain", &self.0.domain)
            .finish()
    }
}

impl ManagementApi {
    /// Create a new istance of the management API.
    pub fn new(domain: &str, api_token: &str) -> Result<Self> {
        let inner = Inner {
            domain: Url::parse(domain).context("Auth0 domain is not a valid url")?,
            api_token: HeaderValue::from_str(&format!("Bearer {}", api_token))
                .context("Auth0 token is not a valid string")?,
            client: Client::new(),
        };
        Ok(Self(Arc::new(inner)))
    }

    /// Contains all the methods to call the `/users` endpoints.
    pub fn users(&self) -> Users {
        let api = self.clone();
        Users { api }
    }

    /// Contains all the methods to call the `/clients` endpoints.
    pub fn clients(&self) -> Clients {
        let api = self.clone();
        Clients { api }
    }

    /// Send a get request to the given endpoint.
    pub(crate) async fn http_get<Q, T>(&self, endpoint: &str, query: Q) -> Result<T>
    where
        Q: Serialize,
        T: DeserializeOwned,
    {
        let mut url = self.0.domain.join(endpoint)?;
        let query = serde_urlencoded::to_string(query)?;
        if !query.is_empty() {
            url.set_query(Some(&query));
        }

        let mut request = Request::new(Method::GET, url);
        request
            .headers_mut()
            .insert(header::AUTHORIZATION, self.0.api_token.clone());

        self.0
            .client
            .execute(request)
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }
}
