use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rjustauth::request::AuthGithubRequest;
use std::path::PathBuf;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

mod rest_auth_controller;
mod user_service;

#[tokio::main]
async fn main() {
    // 设置环境变量 'RUST_LOG' 控制日志级别
    std::env::set_var("RUST_LOG", "DEBUG,rjustauth=DEBUG");
    // $env:RUST_LOG="debug,rjustauth=debug"
    // 初始化 `tracing_subscriber`，将日志输出到控制台
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    // 设置全局的日志订阅器
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // tracing_subscriber::fmt::init();
    // let env = std::env::var("RUST_LOG").unwrap();
    // println!("env: {:?}", env);
    // build our application with a route
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    let app = Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .nest_service("/assets", serve_dir.clone())
        .route(
            "/oauth/render/:source",
            get(rest_auth_controller::render_auth),
        )
        .route("/oauth/callback/:source", get(rest_auth_controller::login))
        .fallback_service(serve_dir);
    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    tracing::debug!("logger ready.");
    tracing::info!("logger ready.");
    tracing::warn!("logger ready.");
    tracing::error!("logger ready.");

    axum::serve(listener, app).await.unwrap();
}
