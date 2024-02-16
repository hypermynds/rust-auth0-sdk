use auth0_sdk::ManagementApi;
use claym::*;
use wiremock::matchers;

use crate::mock::*;

#[tokio::test]
async fn should_list_users() {
    let mock = MockApi::new().await;
    matcher_mgmt_users_list(&mock)
        .respond_with(response_mgmt_users_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let users = mgmt.users();

    let response = assert_ok!(users.list().send().await);
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

    let response = assert_ok!(users.list().page(24).per_page(5).send().await);
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

    let response = assert_ok!(users.list().include_totals(true).send().await);
    assert_eq!(response.users.len(), 2);
    assert_some_eq!(response.start, 0);
    assert_some_eq!(response.length, 14);
    assert_some_eq!(response.total, 14);
    assert_some_eq!(response.limit, 50);
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

    let response = assert_ok!(users.list().sort("date:1").send().await);
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

    let response = assert_ok!(users.list().query("email:\\*@gmail.com").send().await);
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

    let response = assert_ok!(
        users
            .list()
            .field("some")
            .field("random")
            .field("fields")
            .send()
            .await
    );
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

    let response = assert_ok!(
        users
            .list()
            .fields(["some", "random", "fields"])
            .send()
            .await
    );
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

    assert_ok!(users.get(user_id).send().await);
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

    assert_ok!(
        users
            .get(user_id)
            .field("some")
            .field("random")
            .field("fields")
            .send()
            .await
    );
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

    assert_ok!(
        users
            .get(user_id)
            .fields(["some", "random", "fields"])
            .send()
            .await
    );
}
