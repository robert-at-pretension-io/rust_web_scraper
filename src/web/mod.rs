use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

pub async fn start_server() {
    let app = Router::new()
        .route("/documents", get(list_documents))
        .route("/documents", post(create_document))
        .route("/search", post(search_documents))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Web server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_documents() -> &'static str {
    // Implementation for listing documents
    "[]"
}

async fn create_document() -> &'static str {
    // Implementation for creating a document
    "{}"
}

async fn search_documents() -> &'static str {
    // Implementation for searching documents
    "[]"
}
