use std::ops::Deref;

use fake::Fake;
use http::header;
use wiremock::{matchers, Mock, MockBuilder, MockServer, ResponseTemplate};

pub struct MockApi {
    api_token: String,
    server: MockServer,
}

impl MockApi {
    /// Create a new mocked auth0 api.
    pub async fn new() -> Self {
        let api_token = 20_usize.fake();
        let server = MockServer::start().await;
        Self { api_token, server }
    }

    /// Get the api token.
    pub fn api_token(&self) -> &str {
        &self.api_token
    }

    /// Get the api address.
    pub fn domain(&self) -> String {
        self.server.uri()
    }
}

impl Deref for MockApi {
    type Target = MockServer;

    fn deref(&self) -> &Self::Target {
        &self.server
    }
}

// responses
// ----------------------------------------------------------------------------
pub fn response_mgmt_client() -> ResponseTemplate {
    const BODY: &[u8] = include_bytes!("../../testdata/mgmt/client.json");
    json_response_template(BODY)
}

pub fn response_mgmt_clients_list() -> ResponseTemplate {
    const BODY: &[u8] = include_bytes!("../../testdata/mgmt/clients_list.json");
    json_response_template(BODY)
}

pub fn response_mgmt_clients_paged_list() -> ResponseTemplate {
    const BODY: &[u8] = include_bytes!("../../testdata/mgmt/clients_paged_list.json");
    json_response_template(BODY)
}

pub fn response_mgmt_user() -> ResponseTemplate {
    const BODY: &[u8] = include_bytes!("../../testdata/mgmt/user.json");
    json_response_template(BODY)
}

pub fn response_mgmt_users_list() -> ResponseTemplate {
    const BODY: &[u8] = include_bytes!("../../testdata/mgmt/users_list.json");
    json_response_template(BODY)
}

pub fn response_mgmt_users_paged_list() -> ResponseTemplate {
    const BODY: &[u8] = include_bytes!("../../testdata/mgmt/users_paged_list.json");
    json_response_template(BODY)
}

// matchers
// ----------------------------------------------------------------------------
pub fn matcher_mgmt_clients_list(api: &MockApi) -> MockBuilder {
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/clients"))
        .and(matchers::header(
            header::AUTHORIZATION,
            format!("Bearer {}", api.api_token()),
        ))
}

pub fn matcher_mgmt_clients_get(api: &MockApi, id: &str) -> MockBuilder {
    Mock::given(matchers::method("GET"))
        .and(matchers::path(format!("/api/v2/clients/{id}")))
        .and(matchers::header(
            header::AUTHORIZATION,
            format!("Bearer {}", api.api_token()),
        ))
}

pub fn matcher_mgmt_users_list(api: &MockApi) -> MockBuilder {
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/users"))
        .and(matchers::header(
            header::AUTHORIZATION,
            format!("Bearer {}", api.api_token()),
        ))
}

pub fn matcher_mgmt_users_get(api: &MockApi, id: &str) -> MockBuilder {
    Mock::given(matchers::method("GET"))
        .and(matchers::path(format!("/api/v2/users/{id}")))
        .and(matchers::header(
            header::AUTHORIZATION,
            format!("Bearer {}", api.api_token()),
        ))
}

fn json_response_template(data: &[u8]) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(data, "application/json")
}
