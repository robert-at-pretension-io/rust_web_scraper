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
        .route("/", get(list_documents))
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


