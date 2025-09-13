use axum::{routing::{post, get}, Router};

use crate::{errors::ApplicationError, handlers::{help_page, transform_case_handler}};

pub async fn create_server(addr: &str) -> Result<(), ApplicationError> {
    let app = Router::new()
        .route("/", get(help_page))
        .route("/transform", post(transform_case_handler));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
