use axum::{http::StatusCode, Json};
use serde::Deserialize;

use crate::parsing::{change_tag_content_case, TargetCase};

#[derive(Deserialize)]
pub struct ReqBody {
    transform: String,
    html: String,
    selector: Option<String>,
}

#[axum::debug_handler]
pub async fn transform_case_handler(Json(payload): Json<ReqBody>) -> (StatusCode, String) {
    let ReqBody { html, transform, selector } = payload;
    let target_case = match transform.to_lowercase().trim().as_ref() {
        "uppercase" => TargetCase::UpperCase,
        "lowercase" => TargetCase::LowerCase,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                "Invalid value received for target case.".to_owned(),
            );
        }
    };

    match change_tag_content_case(
        &html,
        &selector.unwrap_or("p".to_owned()),
        target_case
    ) {
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
