use auth0_sdk::AuthenticationApi;
use claym::*;
use serde_json::json;
use wiremock::matchers;

use crate::mock::*;

#[tokio::test]
async fn fail_to_get_token_with_client_credentials_missing_secret() {
    let client_id = "xxxyyyzzz";
    let audience = "https://domain.auth0.com/users";

    let mock = MockApi::new().await;
    let auth = assert_ok!(AuthenticationApi::new(&mock.domain(), client_id));
    assert_err!(auth.get_token(audience));
}

#[tokio::test]
async fn should_get_token_with_client_credentials() {
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

#[tokio::test]
async fn fail_to_get_token_with_authorization_code_missing_secret() {
    let client_id = "xxxyyyzzz";
    let code = "code_of_xxxyyyzzz";

    let mock = MockApi::new().await;
    let auth = assert_ok!(AuthenticationApi::new(&mock.domain(), client_id));
    assert_err!(auth.get_token_with_auth_code(code));
}

#[tokio::test]
async fn should_get_token_with_authorization_code() {
    let client_id = "xxxyyyzzz";
    let client_secret = "secret_of_xxxyyyzzz";
    let code = "code_of_xxxyyyzzz";

    let mock = MockApi::new().await;
    matcher_get_token()
        .and(matchers::body_json(json!({
            "grant_type": "authorization_code",
            "client_id": &client_id,
            "client_secret": &client_secret,
            "code": code,
        })))
        .respond_with(response_auth_tokens())
        .mount(&mock)
        .await;

    let auth = assert_ok!(AuthenticationApi::with_client_secret(
        &mock.domain(),
        client_id,
        client_secret
    ));
    let request = assert_ok!(auth.get_token_with_auth_code(code));
    let response = assert_ok!(request.send().await);

    assert!(!response.access_token.is_empty());
    assert!(!response.token_type.is_empty());
    assert!(!assert_some!(response.refresh_token).is_empty());
    assert!(!assert_some!(response.id_token).is_empty());
}

#[tokio::test]
async fn should_get_token_with_authorization_code_pkce() {
    let client_id = "xxxyyyzzz";
    let code = "code_of_xxxyyyzzz";
    let code_verifier = "code_verifier_of_xxxyyyzzz";

    let mock = MockApi::new().await;
    matcher_get_token()
        .and(matchers::body_json(json!({
            "grant_type": "authorization_code",
            "client_id": &client_id,
            "code": code,
            "code_verifier": code_verifier,
        })))
        .respond_with(response_auth_tokens())
        .mount(&mock)
        .await;

    let auth = assert_ok!(AuthenticationApi::new(&mock.domain(), client_id));
    let request = assert_ok!(auth.get_token_with_auth_code_pkce(code, code_verifier));
    let response = assert_ok!(request.send().await);

    assert!(!response.access_token.is_empty());
    assert!(!response.token_type.is_empty());
    assert!(!assert_some!(response.refresh_token).is_empty());
    assert!(!assert_some!(response.id_token).is_empty());
}

#[tokio::test]
async fn should_login_with_user_password() {
    let client_id = "xxxyyyzzz";

    let username = "~~username~~";
    let password = "~~password~~";

    let mock = MockApi::new().await;
    matcher_get_token()
        .and(matchers::body_json(json!({
            "grant_type": "http://auth0.com/oauth/grant-type/password-realm",
            "client_id": &client_id,
            "username": username,
            "password": password,
        })))
        .respond_with(response_auth_tokens())
        .mount(&mock)
        .await;

    let auth = assert_ok!(AuthenticationApi::new(&mock.domain(), client_id,));
    let response = assert_ok!(auth.login(username, password).send().await);

    assert!(!response.access_token.is_empty());
    assert!(!response.token_type.is_empty());
    assert!(!assert_some!(response.refresh_token).is_empty());
    assert!(!assert_some!(response.id_token).is_empty());
}

#[tokio::test]
async fn should_login_with_user_password_with_custom_parameters() {
    let client_id = "xxxyyyzzz";

    let username = "~~username~~";
    let password = "~~password~~";

    let audience = "https://domain.auth0.com/users";
    let realm = "dbconnection";

    let mock = MockApi::new().await;
    matcher_get_token()
        .and(matchers::body_json(json!({
            "grant_type": "http://auth0.com/oauth/grant-type/password-realm",
            "client_id": &client_id,
            "audience": audience,
            "username": username,
            "password": password,
            "scope": "profile photos contacts",
            "realm": realm,
        })))
        .respond_with(response_auth_tokens())
        .mount(&mock)
        .await;

    let auth = assert_ok!(AuthenticationApi::new(&mock.domain(), client_id,));
    let response = assert_ok!(
        auth.login(username, password)
            .audience(audience)
            .scopes(["profile", "photos", "contacts"])
            .realm(realm)
            .send()
            .await
    );

    assert!(!response.access_token.is_empty());
    assert!(!response.token_type.is_empty());
    assert!(!assert_some!(response.refresh_token).is_empty());
    assert!(!assert_some!(response.id_token).is_empty());
}
