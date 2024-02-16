//! Implementation of the Users methods of the Management API.

use anyhow::Result;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};

use crate::{models, ManagementApi};

/// Search engine version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchEngine {
    V1,
    V2,
    V3,
}

/// This struct provides an implementation of the Users methods of the Management API.
pub struct Users {
    pub(crate) api: ManagementApi,
}

impl Users {
    /// Retrieve the list of users, implementation of [`/api/v2/users`] endpoint.
    ///
    /// [`/api/v2/users`]: https://auth0.com/docs/api/management/v2/users/get-users
    pub fn list(&self) -> ListUsersBuilder {
        let mut builder = ListUsersBuilder::default();
        builder.api(self.api.clone());
        builder
    }

    /// Retrieve user details, implementation of [`/api/v2/users/{id}`] endpoint.
    ///
    /// [`/api/v2/users/{id}`]: https://auth0.com/docs/api/management/v2/users/get-users-by-id
    pub fn get<T: Into<String>>(&self, id: T) -> GetUserBuilder {
        let mut builder = GetUserBuilder::default();
        builder.api(self.api.clone()).id(id);
        builder
    }
}

/// Retrieve details of users.
#[serde_as]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Builder, Debug, Serialize)]
#[builder(build_fn(error = "anyhow::Error"))]
pub struct ListUsers {
    #[builder(private)]
    #[serde(skip)]
    api: ManagementApi,
    /// Page index of the results to return. First page is 0.
    #[builder(setter(strip_option), default)]
    page: Option<usize>,
    /// Number of results per page. Paging is disabled if parameter not sent.
    #[builder(setter(strip_option), default)]
    per_page: Option<usize>,
    /// Return results inside an object that contains the total result count. Default is `false`.
    #[builder(setter(strip_option), default)]
    include_totals: Option<bool>,
    /// Field to sort by.
    #[builder(setter(strip_option, into), default)]
    sort: Option<String>,
    /// Connection filter.
    #[builder(setter(strip_option, into), default)]
    connection: Option<String>,
    /// List of fields to include or exclude.
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    #[builder(setter(custom), default)]
    fields: Vec<String>,
    /// Whether specified fields are to be included.
    #[builder(setter(strip_option), default)]
    include_fields: Option<bool>,
    /// Query in Lucene query string syntax.
    #[builder(setter(strip_option, into, name = "query"), default)]
    q: Option<String>,
    /// The version of the search engine.
    #[builder(setter(strip_option), default)]
    search_engine: Option<SearchEngine>,
}

/// Response for [`ListUsers`].
#[derive(Debug, Clone, Deserialize)]
pub struct ListUsersResponse {
    /// List of users.
    pub users: Vec<models::User>,
    /// Page offset.
    pub start: Option<usize>,
    /// Maximum number of items per page.
    pub limit: Option<usize>,
    /// Number of items per page.
    pub length: Option<usize>,
    /// Total number of elements.
    pub total: Option<usize>,
}

impl ListUsers {
    /// Send the API request.
    pub async fn send(self) -> Result<ListUsersResponse> {
        let endpoint = "/api/v2/users";
        if self.include_totals.unwrap_or(false) {
            self.api.http_get(endpoint, &self).await
        } else {
            let users = self.api.http_get(endpoint, &self).await?;
            Ok(ListUsersResponse {
                start: None,
                limit: None,
                length: None,
                total: None,
                users,
            })
        }
    }
}

impl ListUsersBuilder {
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

/// Retrieve user details.
#[serde_as]
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(skip_serializing_if = "Vec::is_empty")],
)]
#[derive(Builder, Debug, Serialize)]
#[builder(build_fn(error = "anyhow::Error"))]
pub struct GetUser {
    #[builder(private)]
    #[serde(skip)]
    api: ManagementApi,
    /// ID of the user to retrieve.
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

/// Response for [`GetUser`].
pub type GetUserResponse = models::User;

impl GetUser {
    /// Send the API request.
    pub async fn send(self) -> Result<GetUserResponse> {
        let endpoint = format!("/api/v2/users/{}", self.id);
        self.api.http_get(&endpoint, &self).await
    }
}

impl GetUserBuilder {
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
