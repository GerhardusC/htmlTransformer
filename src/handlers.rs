use axum::{http::{HeaderName, HeaderValue, StatusCode}, response::{IntoResponse, Response}, Json};
use serde::Deserialize;

use crate::parsing::{transform_case, TargetCase};

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

    match transform_case(
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
    response.headers_mut().insert(HeaderName::from_static("content-type"), HeaderValue::from_static("text/markdown"));

    response
}
