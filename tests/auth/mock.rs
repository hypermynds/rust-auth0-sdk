use std::ops::Deref;

use wiremock::{matchers, Mock, MockBuilder, MockServer, ResponseTemplate};

pub struct MockApi {
    server: MockServer,
}

impl MockApi {
    /// Create a new mocked auth0 api.
    pub async fn new() -> Self {
        let server = MockServer::start().await;
        Self { server }
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
// matchers
// ----------------------------------------------------------------------------
pub fn matcher_get_token() -> MockBuilder {
    Mock::given(matchers::method("POST")).and(matchers::path("/oauth/token"))
}

pub fn matcher_get_device_code() -> MockBuilder {
    Mock::given(matchers::method("POST")).and(matchers::path("/oauth/device/code"))
}

// responses
// ----------------------------------------------------------------------------
pub fn response_auth_tokens() -> ResponseTemplate {
    const BODY: &[u8] = include_bytes!("../../testdata/auth/tokens.json");
    json_response_template(BODY)
}

pub fn response_auth_device_code() -> ResponseTemplate {
    const BODY: &[u8] = include_bytes!("../../testdata/auth/device_code.json");
    json_response_template(BODY)
}

fn json_response_template(data: &[u8]) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(data, "application/json")
}
