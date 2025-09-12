use axum::{routing::post, Router};

use crate::{errors::ApplicationError, handlers::transform_case_handler};

pub async fn create_server(addr: &str) -> Result<(), ApplicationError> {
    let app = Router::new()
        .route("/transform", post(transform_case_handler));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
