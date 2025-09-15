use axum::{routing::{get, get_service, post}, Router};

use crate::{errors::ApplicationError, handlers::{help_page, transform_case_handler}};
use tower_http::services::ServeDir;

pub async fn create_server(addr: &str) -> Result<(), ApplicationError> {
    let app = Router::new()
        .route("/", get(help_page))
        .route("/transform", post(transform_case_handler));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn routes() -> Router {
    let app = Router::new()
        .route("/transform", post(transform_case_handler));
    app
}

fn routes_static(serve_dir: &str) -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(serve_dir)))
}
