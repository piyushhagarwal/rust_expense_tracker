use axum::http::StatusCode;
use reqwest::Client;
use serde::Serialize;

async fn create_user(user_name: String, user_id : i32) {
    let base_url = "http://localhost:3001";

    let client = Client::new();

    #[derive(Serialize)]
    struct body_create_user{
        user_name : String,
        user_id : i32
    }

    let data = body_create_user{
        user_name,
        user_id
    };

    let body_data = serde_json::to_string(&data).unwrap();

    // Make the POST request to your API endpoint
    let response = client
        .post(format!("{base_url}/createuser"))
        .header("Content-Type", "application/json") // Set the Content-Type header
        .body(body_data)
        .send()
        .await
        .unwrap();

}

async fn delete_user(user_id : i32){
    let base_url = "http://localhost:3001";

    let client = Client::new();

    // Make the DELETE request to your API endpoint
    let delete_user = client
    .delete(format!("{base_url}/{user_id}"))
    .send()
    .await
    .unwrap();
}

#[tokio::test]
async fn test_get_user() {
    
    let base_url = "http://localhost:3001";
    let user_id = 3;

    let client = Client::new();

    // Make the GET request to your API endpoint
    let response = client
        .get(format!("{base_url}/{user_id}"))
        .send()
        .await
        .unwrap();

    // Check the status code of the response
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_user(){

    let base_url = "http://localhost:3001";
    let user_id = 874823784;

    let client = Client::new();

    #[derive(Serialize)]
    struct body_create_user{
        user_name : String,
        user_id : i32
    }

    let data = body_create_user{
        user_name : "test".to_owned(),
        user_id
    };

    let body_data = serde_json::to_string(&data).unwrap();

    // Make the POST request to your API endpoint
    let response = client
        .post(format!("{base_url}/createuser"))
        .header("Content-Type", "application/json") // Set the Content-Type header
        .body(body_data)
        .send()
        .await
        .unwrap();

    // Check the status code of the response
    assert_eq!(response.status(), StatusCode::OK);

    delete_user(user_id).await;  
}

#[tokio::test]
async fn test_delete_user(){

    let base_url = "http://localhost:3001";
    let user_id = 874823785;

    let client = Client::new();

    create_user("user_name".to_string(), user_id).await;

    // Make the DELETE request to your API endpoint
    let response = client
        .delete(format!("{base_url}/{user_id}"))
        .send()
        .await
        .unwrap();

    // Check the status code of the response
    assert_eq!(response.status(), StatusCode::OK);  
}

#[tokio::test]
async fn test_add_balance(){

    let base_url = "http://localhost:3001";
    let user_id = 59843834;

    let client = Client::new();

    create_user("user_name".to_string(), user_id).await;

    #[derive(Serialize)]
    struct body_add_balance{
        amount : f64
    }

    let data = body_add_balance{
        amount : 200.00
    };

    let body_data = serde_json::to_string(&data).unwrap();

    let response = client
        .post(format!("{base_url}/{user_id}/addbalance"))
        .header("Content-Type", "application/json") // Set the Content-Type header
        .body(body_data)
        .send()
        .await
        .unwrap();

    // Check the status code of the response
    assert_eq!(response.status(), StatusCode::OK);

    delete_user(user_id).await;


}