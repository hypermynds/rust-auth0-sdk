//! Implementation of the Clients methods of the Management API.

use anyhow::Result;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};

use crate::{models, ManagementApi};

const CLIENTS_ENDPOINT: &str = "/api/v2/clients";

/// This struct provides an implementation of the Clients methods of the Management API.
pub struct Clients {
    pub(crate) api: ManagementApi,
}

impl Clients {
    /// Retrieve the list of clients, implementation of [`/api/v2/clients`] endpoint.
    ///
    /// [`/api/v2/clients`]: https://auth0.com/docs/api/management/v2/clients/get-clients
    pub fn list(&self) -> ListClientsBuilder {
        let mut builder = ListClientsBuilder::default();
        builder.api(self.api.clone());
        builder
    }

    /// Retrieve client details, implementation of [`/api/v2/clients/{id}`] endpoint.
    ///
    /// [`/api/v2/clients/{id}`]: https://auth0.com/docs/api/management/v2/clients/get-clients-by-id
    pub fn get<T: Into<String>>(&self, id: T) -> GetClientBuilder {
        let mut builder = GetClientBuilder::default();
        builder.api(self.api.clone()).id(id);
        builder
    }
}

/// Retrieve clients (applications and SSO integrations) matching provided filters.
#[serde_as]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Builder, Debug, Serialize)]
#[builder(build_fn(private, error = "anyhow::Error"))]
pub struct ListClients {
    #[builder(private)]
    #[serde(skip)]
    api: ManagementApi,
    /// List of fields to include or exclude.
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    #[builder(setter(custom), default)]
    fields: Vec<String>,
    /// Whether specified fields are to be included.
    #[builder(setter(strip_option), default)]
    include_fields: Option<bool>,
    /// Page index of the results to return. First page is 0.
    #[builder(setter(strip_option), default)]
    page: Option<usize>,
    /// Number of results per page. Paging is disabled if parameter not sent.
    #[builder(setter(strip_option), default)]
    per_page: Option<usize>,
    /// Return results inside an object that contains the total result count. Default is `false`.
    #[builder(setter(strip_option), default)]
    include_totals: Option<bool>,
    /// Optional filter on the global client parameter.
    #[builder(setter(strip_option), default)]
    is_global: Option<bool>,
    /// Optional filter on whether or not a client is a first-party client.
    #[builder(setter(strip_option), default)]
    is_first_party: Option<bool>,
    /// Optional filter by a comma-separated list of application types.
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    #[builder(setter(custom), default)]
    app_type: Vec<String>,
}

/// Response for [`ListClients`].
#[derive(Debug, Clone, Deserialize)]
pub struct ListClientsResponse {
    /// List of clients.
    pub clients: Vec<models::Client>,
    /// Page offset.
    pub start: Option<usize>,
    /// Maximum number of items per page.
    pub limit: Option<usize>,
    /// Number of items per page.
    pub length: Option<usize>,
    /// Total number of elements.
    pub total: Option<usize>,
}

impl ListClientsBuilder {
    /// Send the API request.
    pub async fn send(&self) -> Result<ListClientsResponse> {
        let request = self.build()?;
        if request.include_totals.unwrap_or(false) {
            request.api.http_get(CLIENTS_ENDPOINT, &request).await
        } else {
            let clients = request.api.http_get(CLIENTS_ENDPOINT, &request).await?;
            Ok(ListClientsResponse {
                start: None,
                limit: None,
                length: None,
                total: None,
                clients,
            })
        }
    }

    /// Append one element to the list of `fields`.
    pub fn field<T: Into<String>>(&mut self, field: T) -> &mut Self {
        self.fields.get_or_insert_with(Vec::new).push(field.into());
        self
    }

    /// Append the contents of iterator to the list of `fields`.
    pub fn fields<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.fields
            .get_or_insert_with(Vec::new)
            .extend(iter.into_iter().map(Into::into));
        self
    }

    /// Append one element to the list of `app_type`.
    pub fn app_type<T: Into<String>>(&mut self, app_type: T) -> &mut Self {
        self.app_type
            .get_or_insert_with(Vec::new)
            .push(app_type.into());
        self
    }

    /// Append the contents of iterator to the list of `app_type`.
    pub fn app_types<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.app_type
            .get_or_insert_with(Vec::new)
            .extend(iter.into_iter().map(Into::into));
        self
    }
}

/// Retrieve client details.
#[serde_as]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Builder, Debug, Serialize)]
#[builder(build_fn(private, error = "anyhow::Error"))]
pub struct GetClient {
    #[builder(private)]
    #[serde(skip)]
    api: ManagementApi,
    /// ID of the client to retrieve.
    #[builder(private, setter(into))]
    #[serde(skip)]
    id: String,
    /// List of fields to include or exclude.
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    #[builder(setter(custom), default)]
    fields: Vec<String>,
    /// Whether specified fields are to be included.
    #[builder(setter(strip_option), default)]
    include_fields: Option<bool>,
}

/// Response for [`GetClient`].
pub type GetClientResponse = models::Client;

impl GetClientBuilder {
    /// Send the API request.
    pub async fn send(&self) -> Result<GetClientResponse> {
        let request = self.build()?;
        let endpoint = format!("{}/{}", CLIENTS_ENDPOINT, request.id);
        request.api.http_get(&endpoint, &request).await
    }

    /// Append one element to the list of fields.
    pub fn field<T: Into<String>>(&mut self, field: T) -> &mut Self {
        self.fields.get_or_insert_with(Vec::new).push(field.into());
        self
    }

    /// Append the contents of iterator to the list of fields.
    pub fn fields<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.fields
            .get_or_insert_with(Vec::new)
            .extend(iter.into_iter().map(Into::into));
        self
    }
}
