use axum::{
    extract::{Json, Query},
    response::IntoResponse,
};
use log;
use my_attribute::simple_attribute;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NameQuery {
    name: Option<String>,
}

pub async fn hello_world(Query(query): Query<NameQuery>) -> String {
    format!(
        "Hello, {}!",
        query.name.unwrap_or_else(|| "world".to_string())
    )
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    name: String,
    age: u32,
}
#[simple_attribute]
pub async fn create_user(Json(payload): Json<UserData>) -> impl IntoResponse {
    log::info!("Received user data: {:?}", payload);
    // Here you would typically save the user data to a database
    // For this example, we'll just return the data we received
    Json(payload)
}
