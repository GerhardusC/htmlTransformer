use axum::{Json, http::StatusCode};

use crate::parsing::TransformCaseInput;

#[axum::debug_handler]
pub async fn transform_case_handler(
    Json(payload): Json<TransformCaseInput>,
) -> (StatusCode, String) {
    if !payload.validate_transform() {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid value received for target case.".to_owned(),
        );
    }

    let transformed_result = payload.transform_case();

    return if let Ok(x) = transformed_result {
        (StatusCode::OK, x)
    } else {
        (
            StatusCode::BAD_REQUEST,
            "Failed to parse html content".to_owned(),
        )
    };
}
