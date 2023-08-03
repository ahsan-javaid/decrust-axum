use axum::{
  http::StatusCode,
  Json,
  extract::Query
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

pub async fn get_users() -> &'static str {
  "Hello, World!"
}

pub async fn create_users(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  Query(param): Query<HashMap<String, String>>,
  Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
  param.get("my");
  // insert your application logic here
  let user = User {
      id: 1337,
      username: payload.username,
  };

  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(user))
}