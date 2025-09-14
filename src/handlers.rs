use axum::{
    Json,
    http::{HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

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

#[axum::debug_handler]
pub async fn help_page() -> Response {
    let mut response = "# Case Transformer
This application provides a single endpoint to which you can make requests to transform the case of an XML/HTML tree based on a CSS selector. If a CSS selector is passed, all matching tags will have their contents' case transformed. By default if no selector is passed, the contents of all paragraph tags is transformed.

### POST `/transform`
Transform the contents of all elements matching a CSS selector to a specified case.

**Headers**:
\"content-type\": \"application/json\"

**Body**:
```ts
{
    type RequestBody = {
        transform: \"uppercase\" | \"lowercase\",
        html: String,
        selector?: String | undefined
    }
}
```".into_response();
    response.headers_mut().insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/markdown"),
    );

    response
}
