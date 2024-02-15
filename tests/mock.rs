use std::ops::Deref;

use fake::Fake;
use http::header;
use wiremock::{matchers, Mock, MockBuilder, MockServer, ResponseTemplate};

const MGMT_USERS_LIST: &[u8] = include_bytes!("../testdata/management/users_list.json");
const MGMT_USERS_PAGED_LIST: &[u8] = include_bytes!("../testdata/management/users_paged_list.json");

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

pub fn response_mgmt_users_list() -> ResponseTemplate {
    json_response_template(MGMT_USERS_LIST)
}

pub fn response_mgmt_users_paged_list() -> ResponseTemplate {
    json_response_template(MGMT_USERS_PAGED_LIST)
}

pub fn matcher_mgmt_users_list(api: &MockApi) -> MockBuilder {
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/users"))
        .and(matchers::header(
            header::AUTHORIZATION,
            format!("Bearer {}", api.api_token()),
        ))
}

fn json_response_template(data: &[u8]) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(data, "application/json")
}
