use axum::{routing::post, Router};

use crate::{errors::ApplicationError, handlers::transform_case_handler};

pub async fn create_server(addr: &str) -> Result<(), ApplicationError> {
    // build our application with a single route
    let app = Router::new()
        .route("/transform", post(transform_case_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
