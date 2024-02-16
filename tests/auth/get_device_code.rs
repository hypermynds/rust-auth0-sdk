use auth0_sdk::AuthenticationApi;
use claym::*;
use serde_json::json;
use wiremock::matchers;

use crate::mock::*;

#[tokio::test]
async fn get_device_code() {
    let client_id = "xxxyyyzzz";

    let mock = MockApi::new().await;
    matcher_get_device_code()
        .and(matchers::body_json(json!({
            "client_id": &client_id
        })))
        .respond_with(response_auth_device_code())
        .mount(&mock)
        .await;

    let auth = assert_ok!(AuthenticationApi::new(&mock.domain(), client_id,));
    assert_ok!(auth.get_device_code().send().await);
}

#[tokio::test]
async fn get_device_code_with_custom_parameters() {
    let client_id = "xxxyyyzzz";
    let audience = "https://domain.auth0.com/users";

    let mock = MockApi::new().await;
    matcher_get_device_code()
        .and(matchers::body_json(json!({
            "client_id": &client_id,
            "audience": audience,
            "scope": "openid profile",
        })))
        .respond_with(response_auth_device_code())
        .mount(&mock)
        .await;

    let auth = assert_ok!(AuthenticationApi::new(&mock.domain(), client_id,));
    assert_ok!(
        auth.get_device_code()
            .audience(audience)
            .scopes(["openid", "profile"])
            .send()
            .await
    );
}
