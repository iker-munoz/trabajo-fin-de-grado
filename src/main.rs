// Imports
// External crates
use axum::Router;
use tokio::net::TcpListener;
use tokio::try_join;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

// Project modules
mod error;

use error::ServerError;

#[tokio::main]
async fn main() {
    if let Err(e) = try_join!(run_api(), run_webserver()) {
        println!("{}", e);
    };
}

async fn run_api() -> Result<(), ServerError> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:5000")
        .await
        .map_err(|e| ServerError::SetupError(format!("Error  binding API port: {}", e)))?;

    let cors: CorsLayer = CorsLayer::new().allow_headers(Any);
    let router: Router = Router::new().layer(cors);

    axum::serve(listener, router)
        .await
        .map_err(|e| ServerError::SetupError(format!("Error starting the API: {}", e)))?;

    Ok(())
}

async fn run_webserver() -> Result<(), ServerError> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080")
        .await
        .map_err(|e| ServerError::SetupError(format!("Error binding webserver port: {}", e)))?;

    let file_server: ServeDir<_> = ServeDir::new("site")
        .append_index_html_on_directories(true)
        .not_found_service(ServeFile::new("site/error.html"));

    let router: Router = Router::new().fallback_service(file_server);

    axum::serve(listener, router)
        .await
        .map_err(|e| ServerError::SetupError(format!("Error starting the webserver: {}", e)))?;

    Ok(())
}
