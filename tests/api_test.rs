use axum::http::StatusCode;
use serde_json::Value;
use reqwest::Client;

#[tokio::test]
async fn test_get_user() {
    // Replace "http://localhost:3000" with the actual URL of your running Axum application.
    let base_url = "http://localhost:3000";
    let user_id = 123;

    let client = Client::new();

    // Make the GET request to your API endpoint
    let response = client
        .get("localhost:3001/3")
        .send()
        .await
        .unwrap();

    // Check the status code of the response
    assert_eq!(response.status(), StatusCode::OK);
    println!("{:#?}",response)

    // You can also check the response body if needed
    // For example, if you know the JSON structure of the response:
    // let body: Value = response.json().await.unwrap();
    // Perform assertions on the JSON data
    // assert_eq!(body["user_id"].as_i64(), Some(user_id));
}