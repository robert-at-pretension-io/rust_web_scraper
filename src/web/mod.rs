use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

pub async fn start_server(pool: Arc<SqlitePool>) {
    let app = Router::new()
        .route("/", get(|| async { Html("Welcome to Web Scraper") }))
        .route("/documents", get(list_documents))
        .route("/search", get(search_page))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Web server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list_documents(
    Query(params): Query<SearchParams>,
    State(pool): State<Arc<SqlitePool>>,
) -> Html<String> {
    let query = params.q.unwrap_or_default();
    
    let documents = if query.is_empty() {
        sqlx::query_as!(
            crate::models::Document,
            r#"SELECT id, title, content, url, created_at as "created_at: chrono::DateTime<Utc>",
               updated_at as "updated_at: chrono::DateTime<Utc>"
               FROM documents ORDER BY created_at DESC LIMIT 100"#
        )
        .fetch_all(&*pool)
        .await
        .unwrap_or_default()
    } else {
        let like_pattern = format!("%{}%", query);
        sqlx::query_as!(
            crate::models::Document,
            r#"SELECT id, title, content, url, created_at as "created_at: chrono::DateTime<Utc>",
               updated_at as "updated_at: chrono::DateTime<Utc>"
               FROM documents 
               WHERE title LIKE ? OR content LIKE ? 
               ORDER BY created_at DESC LIMIT 100"#,
            like_pattern,
            like_pattern
        )
        .fetch_all(&*pool)
        .await
        .unwrap_or_default()
    };

    // Simple HTML response
    let mut html = String::from(r#"
        <html>
            <head><title>Documents</title></head>
            <body>
                <h1>Cached Documents</h1>
                <form method="get">
                    <input type="text" name="q" value="">
                    <button type="submit">Search</button>
                </form>
                <table border="1">
                    <tr>
                        <th>ID</th>
                        <th>Title</th>
                        <th>URL</th>
                        <th>Created</th>
                    </tr>
    "#);

    for doc in documents {
        html.push_str(&format!(r#"
            <tr>
                <td>{}</td>
                <td>{}</td>
                <td><a href="{}" target="_blank">{}</a></td>
                <td>{}</td>
            </tr>
        "#, doc.id, doc.title, doc.url, doc.url, doc.created_at));
    }

    html.push_str("</table></body></html>");
    Html(html)
}

async fn search_page(
    Query(params): Query<SearchParams>,
    State(pool): State<Arc<SqlitePool>>,
) -> Html<String> {
    let query = params.q.unwrap_or_default();
    
    let mut html = String::from(r#"
        <html>
            <head><title>Search</title></head>
            <body>
                <h1>Search Documents</h1>
                <form method="get">
                    <input type="text" name="q" value="">
                    <button type="submit">Search</button>
                </form>
    "#);

    if !query.is_empty() {
        let like_pattern = format!("%{}%", query);
        let results = sqlx::query!(
            r#"SELECT d.id, d.title, d.content, d.url, d.created_at
               FROM documents d
               WHERE d.title LIKE ? OR d.content LIKE ?
               ORDER BY d.created_at DESC LIMIT 10"#,
            like_pattern,
            like_pattern
        )
        .fetch_all(&*pool)
        .await
        .unwrap_or_default();

        for result in results {
            html.push_str(&format!(r#"
                <div style="margin: 20px 0; padding: 10px; border-left: 3px solid blue;">
                    <h3><a href="{}" target="_blank">{}</a></h3>
                    <p>{}</p>
                    <small>ID: {} | Created: {}</small>
                </div>
            "#, result.url, result.title, result.content, result.id, result.created_at));
        }
    }

    html.push_str("</body></html>");
    Html(html)
}

