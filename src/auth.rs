//! Implementation of the Authentication API.

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use derive_builder::Builder;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};
use serde_with::{formats::SpaceSeparator, serde_as, StringWithSeparator};

use crate::models;

const GET_TOKEN_ENDPOINT: &str = "/oauth/token";
const GRANT_TYPE_CLIENT_CREDENTIALS: &str = "client_credentials";
const GRANT_TYPE_RESOURCE_OWNED_PASSWORD: &str = "http://auth0.com/oauth/grant-type/password-realm";

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

    /// Request an access using the client's credentials, implementation of [client credentials
    /// flow].
    ///
    /// [client credentials flow]: https://auth0.com/docs/api/authentication#client-credentials-flow
    pub fn get_token<T: Into<String>>(&self, audience: T) -> Result<ClientCredentialsFlow<'_>> {
        Ok(ClientCredentialsFlow {
            api: self,
            grant_type: GRANT_TYPE_CLIENT_CREDENTIALS,
            client_id: &self.0.client_id,
            client_secret: self
                .0
                .client_secret
                .as_ref()
                .ok_or_else(|| anyhow!("Missing client_secret"))?,
            audience: audience.into(),
        })
    }

    /// Log in using the user's credentials, implementation of [resource owned password flow].
    ///
    /// [resource owned password flow]: https://auth0.com/docs/api/authentication?javascript#resource-owner-password
    pub fn login<U, P>(&self, username: U, password: P) -> ResourceOwnerPasswordFlowBuilder
    where
        U: Into<String>,
        P: Into<String>,
    {
        let mut builder = ResourceOwnerPasswordFlowBuilder::default();
        builder
            .api(self.clone())
            .grant_type(GRANT_TYPE_RESOURCE_OWNED_PASSWORD)
            .client_id(self.0.client_id.clone())
            .client_secret(self.0.client_secret.clone())
            .username(username)
            .password(password);
        builder
    }
}

/// Get an access token by using the client's credentials.
#[derive(Debug, Serialize)]
pub struct ClientCredentialsFlow<'a> {
    #[serde(skip)]
    api: &'a AuthenticationApi,
    /// Denotes the flow you are using.
    grant_type: &'static str,
    /// Application's Client ID.
    client_id: &'a str,
    /// Application's Client Secret.
    client_secret: &'a str,
    /// The unique identifier of the target API you want to access.
    audience: String,
}

impl<'a> ClientCredentialsFlow<'a> {
    /// Send the API request.
    pub async fn send(&self) -> Result<models::AccessToken> {
        self.api.http_post(GET_TOKEN_ENDPOINT, self).await
    }
}

/// Get an access token by using user's credentials.
#[serde_as]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Builder, Debug, Serialize)]
#[builder(build_fn(private, error = "anyhow::Error"))]
pub struct ResourceOwnerPasswordFlow {
    #[builder(private)]
    #[serde(skip)]
    api: AuthenticationApi,
    /// Denotes the flow you are using.
    #[builder(private)]
    grant_type: &'static str,
    /// Application's Client ID.
    #[builder(private)]
    client_id: String,
    /// Application's Client Secret.
    #[builder(private)]
    client_secret: Option<String>,
    /// The unique identifier of the target API you want to access.
    #[builder(setter(strip_option, into), default)]
    audience: Option<String>,
    /// Resource Owner's identifier, such as a username or email address.
    #[builder(private, setter(into))]
    username: String,
    /// Resource Owner's secret.
    #[builder(private, setter(into))]
    password: String,
    /// String value of the different scopes the application is asking for. Multiple scopes are
    /// separated with whitespace.
    #[serde_as(as = "StringWithSeparator::<SpaceSeparator, String>")]
    #[builder(setter(custom), default)]
    scope: Vec<String>,
    /// String value of the realm the user belongs. Set this if you want to add realm support at this grant.
    #[builder(setter(strip_option, into), default)]
    realm: Option<String>,
}

impl ResourceOwnerPasswordFlowBuilder {
    /// Send the API request.
    pub async fn send(&self) -> Result<models::AccessToken> {
        let request = self.build()?;
        request.api.http_post(GET_TOKEN_ENDPOINT, &request).await
    }

    /// Append one element to the list of scope.
    pub fn scope<T: Into<String>>(&mut self, scope: T) -> &mut Self {
        self.scope.get_or_insert_with(Vec::new).push(scope.into());
        self
    }

    /// Append the contents of iterator to the list of scopes.
    pub fn scopes<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.scope
            .get_or_insert_with(Vec::new)
            .extend(iter.into_iter().map(Into::into));
        self
    }
}
