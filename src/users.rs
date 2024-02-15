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
    /// Retrieve the list of users.
    pub fn list(&self) -> ListBuilder {
        let mut builder = ListBuilder::default();
        builder.api(self.api.clone());
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
pub struct List {
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

/// Response for users list
#[derive(Debug, Clone, Deserialize)]
pub struct ListResponse {
    /// List of users
    pub users: Vec<models::User>,
    /// Page offset
    pub start: Option<usize>,
    /// Maximum number of items per page
    pub limit: Option<usize>,
    /// Number of items per page
    pub length: Option<usize>,
    /// Total number of elements
    pub total: Option<usize>,
}

impl List {
    /// Send the API request.
    pub async fn send(self) -> Result<ListResponse> {
        let endpoint = "/api/v2/users";
        if self.include_totals.unwrap_or(false) {
            self.api.http_get(endpoint, &self).await
        } else {
            let users = self.api.http_get(endpoint, &self).await?;
            Ok(ListResponse {
                start: None,
                limit: None,
                length: None,
                total: None,
                users,
            })
        }
    }
}

impl ListBuilder {
    /// Append one element to the list of fields
    pub fn field<T>(&mut self, field: T) -> &mut Self
    where
        T: Into<String>,
    {
        let fields = self.fields.get_or_insert_with(Vec::new);
        fields.push(field.into());
        self
    }

    /// Append the contents of iterator to the list of fields
    pub fn fields<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let fields = self.fields.get_or_insert_with(Vec::new);
        fields.extend(iter.into_iter().map(Into::into));
        self
    }
}
