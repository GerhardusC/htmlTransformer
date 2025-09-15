use axum::{
    Router,
    routing::{get_service, post},
};

use crate::{errors::ApplicationError, handlers::transform_case_handler};
use tower_http::services::ServeDir;

pub async fn create_server(addr: &str, serve_dir: &str) -> Result<(), ApplicationError> {
    let app = Router::new()
        .route("/transform", post(transform_case_handler))
        .fallback_service(get_service(ServeDir::new(serve_dir)));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

