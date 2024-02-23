use axum::extract::{Path, Query};
use axum::http::Response;
use axum::response::{IntoResponse, Redirect};
use rjustauth::base::model::AuthCallback;
use rjustauth::request::AuthGithubRequest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    source: String,
}

pub async fn render_auth(Path(source): Path<String>) -> impl IntoResponse {
    println!("renderAuth");
    // println!("path: {:?}", path);
    println!("source: {:?}", source);
    let auth_request = get_auth_request(&source);

    Redirect::to(auth_request.authorize("state").as_str())
}

pub async fn login(
    Path(source): Path<String>,
    Query(params): Query<AuthCallback>,
) -> impl IntoResponse {
    tracing::debug!("进入callback：{:?}", source);
    tracing::info!("params: {:?}", params);
    let auth_request = get_auth_request(&source);
    // AuthResponse<AuthUser> response = auth_request.login(callback);
    // Redirect::to(auth_request.authorize("state").as_str())
    auth_request.login(params.into());
    Redirect::to("http://localhost:3000")
}

pub fn get_auth_request(source: &str) -> AuthGithubRequest {
    return match source {
        "github" => AuthGithubRequest::new()
            .client_id("2eae3c445939fe3a971b")
            .client_secret("cd947b05ab06bfa99e0f233f930e5601d5ee4c42")
            .redirect_uri("http://localhost:3000/oauth/callback/github")
            .build(),
        _ => panic!("source not found"),
    };
}
