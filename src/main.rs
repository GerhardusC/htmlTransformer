mod errors;
use serde::Deserialize;

use axum::{Router, extract::Json, http::StatusCode, routing::post};
use rocketseed_interview::{TargetCase, change_tag_content_case};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/transform", post(transform_case_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct ReqBody {
    transform: String,
    html: String,
}

#[axum::debug_handler]
async fn transform_case_handler(Json(payload): Json<ReqBody>) -> (StatusCode, String) {
    let ReqBody { html, transform } = payload;
    let target_case = match transform.as_ref() {
        "uppercase" => TargetCase::UpperCase,
        "lowercase" => TargetCase::LowerCase,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                "Invalid value received for target case.".to_owned(),
            );
        }
    };
    match change_tag_content_case(&html, "p", target_case) {
        Ok(x) => {
            return (StatusCode::OK, x);
        }
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                "Failed to parse html content".to_owned(),
            );
        }
    }
}
