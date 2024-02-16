use auth0_sdk::AuthenticationApi;
use claym::*;
use serde_json::json;
use wiremock::matchers;

use crate::mock::*;

#[tokio::test]
async fn get_token_with_client_credentials_grant_request() {
    let client_id = "xxxyyyzzz";
    let client_secret = "secret_of_xxxyyyzzz";
    let audience = "https://domain.auth0.com/users";

    let mock = MockApi::new().await;
    matcher_get_token()
        .and(matchers::body_json(json!({
            "grant_type": "client_credentials",
            "client_id": &client_id,
            "client_secret": &client_secret,
            "audience": &audience,
        })))
        .respond_with(response_auth_tokens())
        .mount(&mock)
        .await;

    let auth = assert_ok!(AuthenticationApi::with_client_secret(
        &mock.domain(),
        client_id,
        client_secret
    ));
    let request = assert_ok!(auth.get_token(audience));
    let response = assert_ok!(request.send().await);

    assert!(!response.access_token.is_empty());
    assert!(!response.token_type.is_empty());
    assert!(!assert_some!(response.refresh_token).is_empty());
    assert!(!assert_some!(response.id_token).is_empty());
}
