use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use calc::{divide, hello, multiply, subtract, sum};
use serde::Deserialize;

#[derive(Deserialize)]
struct CalcValues {
    left: i32,
    right: i32,
}

async fn get_hello(Path(name): Path<String>) -> String {
    hello(name.as_str())
}

async fn get_sum(calc_values: Query<CalcValues>) -> String {
    sum(calc_values.left, calc_values.right).to_string()
}

async fn get_subtract(calc_values: Query<CalcValues>) -> String {
    subtract(calc_values.left, calc_values.right).to_string()
}

async fn get_multiply(calc_values: Query<CalcValues>) -> String {
    multiply(calc_values.left, calc_values.right).to_string()
}

async fn get_divide(calc_values: Query<CalcValues>) -> String {
    divide(calc_values.left, calc_values.right).to_string()
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { hello("World") }))
        .route("/:name", get(get_hello))
        .route("/sum", get(get_sum))
        .route("/subtract", get(get_subtract))
        .route("/multiply", get(get_multiply))
        .route("/divide", get(get_divide));

    // run our app with hyper, listening globally on port 3001
    // Example call: http://localhost:3001/sum?left=2&right=3
    let listener = tokio::net::TcpListener::bind("localhost:3001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
