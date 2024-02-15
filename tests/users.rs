use auth0_sdk::ManagementApi;
use claym::*;
use wiremock::matchers;

use self::mock::*;

mod mock;

#[tokio::test]
async fn should_list_users() {
    let mock = MockApi::new().await;
    matcher_mgmt_users_list(&mock)
        .respond_with(response_mgmt_users_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let request = assert_ok!(users.list().build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.users.len(), 2);
}

#[tokio::test]
async fn should_list_users_with_page() {
    let mock = MockApi::new().await;
    matcher_mgmt_users_list(&mock)
        .and(matchers::query_param("page", "24"))
        .and(matchers::query_param("per_page", "5"))
        .respond_with(response_mgmt_users_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let request = assert_ok!(users.list().page(24).per_page(5).build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.users.len(), 2);
}

#[tokio::test]
async fn should_list_users_with_totals() {
    let mock = MockApi::new().await;
    matcher_mgmt_users_list(&mock)
        .and(matchers::query_param("include_totals", "true"))
        .respond_with(response_mgmt_users_paged_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let request = assert_ok!(users.list().include_totals(true).build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.users.len(), 2);
}

#[tokio::test]
async fn should_list_users_with_sort() {
    let mock = MockApi::new().await;
    matcher_mgmt_users_list(&mock)
        .and(matchers::query_param("sort", "date:1"))
        .respond_with(response_mgmt_users_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let request = assert_ok!(users.list().sort("date:1").build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.users.len(), 2);
}

#[tokio::test]
async fn should_list_users_with_query() {
    let mock = MockApi::new().await;
    matcher_mgmt_users_list(&mock)
        .and(matchers::query_param("q", "email:\\*@gmail.com"))
        .respond_with(response_mgmt_users_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let request = assert_ok!(users.list().query("email:\\*@gmail.com").build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.users.len(), 2);
}

#[tokio::test]
async fn should_list_users_with_fields_given_separately() {
    let mock = MockApi::new().await;
    matcher_mgmt_users_list(&mock)
        .and(matchers::query_param("fields", "some,random,fields"))
        .respond_with(response_mgmt_users_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let request = assert_ok!(users
        .list()
        .field("some")
        .field("random")
        .field("fields")
        .build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.users.len(), 2);
}

#[tokio::test]
async fn should_list_users_with_fields() {
    let mock = MockApi::new().await;
    matcher_mgmt_users_list(&mock)
        .and(matchers::query_param("fields", "some,random,fields"))
        .respond_with(response_mgmt_users_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let request = assert_ok!(users.list().fields(["some", "random", "fields"]).build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.users.len(), 2);
}

#[tokio::test]
async fn should_get_user() {
    let mock = MockApi::new().await;
    let user_id = "auth0|xxxyyyzz";
    matcher_mgmt_users_get(&mock, user_id)
        .respond_with(response_mgmt_user())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let request = assert_ok!(users.get(user_id).build());
    assert_ok!(request.send().await);
}

#[tokio::test]
async fn should_get_user_with_fields_given_separately() {
    let mock = MockApi::new().await;
    let user_id = "auth0|xxxyyyzz";
    matcher_mgmt_users_get(&mock, user_id)
        .and(matchers::query_param("fields", "some,random,fields"))
        .respond_with(response_mgmt_user())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let request = assert_ok!(users
        .get(user_id)
        .field("some")
        .field("random")
        .field("fields")
        .build());
    assert_ok!(request.send().await);
}

#[tokio::test]
async fn should_get_user_with_fields() {
    let mock = MockApi::new().await;
    let user_id = "auth0|xxxyyyzz";
    matcher_mgmt_users_get(&mock, user_id)
        .and(matchers::query_param("fields", "some,random,fields"))
        .respond_with(response_mgmt_user())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let request = assert_ok!(users
        .get(user_id)
        .fields(["some", "random", "fields"])
        .build());
    assert_ok!(request.send().await);
}
