use auth0_sdk::ManagementApi;
use claym::*;
use wiremock::matchers;

use crate::mock::*;

#[tokio::test]
async fn should_list_clients_without_filter() {
    let mock = MockApi::new().await;
    matcher_mgmt_clients_list(&mock)
        .respond_with(response_mgmt_clients_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let clients = mgmt.clients();

    let request = assert_ok!(clients.list().build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.clients.len(), 2);
}

#[tokio::test]
async fn should_list_clients_with_page() {
    let mock = MockApi::new().await;
    matcher_mgmt_clients_list(&mock)
        .and(matchers::query_param("page", "23"))
        .and(matchers::query_param("per_page", "5"))
        .respond_with(response_mgmt_clients_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let clients = mgmt.clients();

    let request = assert_ok!(clients.list().page(23).per_page(5).build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.clients.len(), 2);
}

#[tokio::test]
async fn should_list_clients_with_totals() {
    let mock = MockApi::new().await;
    matcher_mgmt_clients_list(&mock)
        .and(matchers::query_param("include_totals", "true"))
        .respond_with(response_mgmt_clients_paged_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let clients = mgmt.clients();

    let request = assert_ok!(clients.list().include_totals(true).build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.clients.len(), 2);
    assert_some_eq!(response.start, 0);
    assert_some_eq!(response.length, 14);
    assert_some_eq!(response.total, 14);
    assert_some_eq!(response.limit, 50);
}

#[tokio::test]
async fn should_list_clients_with_fields_given_separately() {
    let mock = MockApi::new().await;
    matcher_mgmt_clients_list(&mock)
        .and(matchers::query_param("fields", "some,random,fields"))
        .respond_with(response_mgmt_clients_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let clients = mgmt.clients();

    let request = assert_ok!(clients
        .list()
        .field("some")
        .field("random")
        .field("fields")
        .build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.clients.len(), 2);
}

#[tokio::test]
async fn should_list_clients_with_fields() {
    let mock = MockApi::new().await;
    matcher_mgmt_clients_list(&mock)
        .and(matchers::query_param("fields", "some,random,fields"))
        .respond_with(response_mgmt_clients_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let clients = mgmt.clients();

    let request = assert_ok!(clients.list().fields(["some", "random", "fields"]).build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.clients.len(), 2);
}

#[tokio::test]
async fn should_list_clients_with_additional_properties() {
    let mock = MockApi::new().await;
    matcher_mgmt_clients_list(&mock)
        .and(matchers::query_param("app_type", "regular_web,native"))
        .and(matchers::query_param("is_first_party", "true"))
        .and(matchers::query_param("is_global", "true"))
        .respond_with(response_mgmt_clients_list())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let clients = mgmt.clients();

    let request = assert_ok!(clients
        .list()
        .app_types(["regular_web", "native"])
        .is_first_party(true)
        .is_global(true)
        .build());
    let response = assert_ok!(request.send().await);
    assert_eq!(response.clients.len(), 2);
}

#[tokio::test]
async fn should_get_client() {
    let mock = MockApi::new().await;
    let client_id = "My-Super-Application-Name";
    matcher_mgmt_clients_get(&mock, client_id)
        .respond_with(response_mgmt_client())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let clients = mgmt.clients();

    let request = assert_ok!(clients.get(client_id).build());
    assert_ok!(request.send().await);
}

#[tokio::test]
async fn should_get_client_with_fields_given_separately() {
    let mock = MockApi::new().await;
    let client_id = "My-Super-Application-Name";
    matcher_mgmt_clients_get(&mock, client_id)
        .and(matchers::query_param("fields", "some,random,fields"))
        .respond_with(response_mgmt_client())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let clients = mgmt.clients();

    let request = assert_ok!(clients
        .get(client_id)
        .field("some")
        .field("random")
        .field("fields")
        .build());
    assert_ok!(request.send().await);
}

#[tokio::test]
async fn should_get_client_with_fields() {
    let mock = MockApi::new().await;
    let client_id = "My-Super-Application-Name";
    matcher_mgmt_clients_get(&mock, client_id)
        .and(matchers::query_param("fields", "some,random,fields"))
        .respond_with(response_mgmt_client())
        .mount(&mock)
        .await;

    let mgmt = assert_ok!(ManagementApi::new(&mock.domain(), mock.api_token()));
    let clients = mgmt.clients();

    let request = assert_ok!(clients
        .get(client_id)
        .fields(["some", "random", "fields"])
        .build());
    assert_ok!(request.send().await);
}
