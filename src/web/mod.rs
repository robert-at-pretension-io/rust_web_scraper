use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
    debug_handler,
};
use askama::Template;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Template)]
#[template(path = "documents.html")]
struct DocumentsTemplate {
    documents: Vec<crate::models::Document>,
    query: String,
}

#[derive(Template)]
#[template(path = "logs.html")]
struct LogsTemplate {
    logs: Vec<(String, String, String)>,
    query: String,
    level: String,
}

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate {
    results: Vec<crate::models::SearchResult>,
    query: String,
}

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
    level: Option<String>,
}

pub async fn start_server(pool: Arc<SqlitePool>) {
    let app = Router::new()
        .route("/", get(|| async { axum::response::Redirect::to("/documents") }))
        .route("/documents", get(list_documents))
        .route("/search", get(search_page))
        .route("/logs", get(view_logs))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Web server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn list_documents(
    Query(params): Query<SearchParams>,
    State(pool): State<Arc<SqlitePool>>,
) -> Html<String> {
    let query = params.q.unwrap_or_default();
    
    let documents = if query.is_empty() {
        sqlx::query_as!(
            crate::models::Document,
            "SELECT id, title, content, url, created_at, updated_at FROM documents ORDER BY created_at DESC LIMIT 100"
        )
        .fetch_all(&*pool)
        .await
        .unwrap_or_default()
    } else {
        sqlx::query_as!(
            crate::models::Document,
            "SELECT id, title, content, url, created_at, updated_at FROM documents WHERE title LIKE ? OR content LIKE ? ORDER BY created_at DESC LIMIT 100",
            format!("%{}%", query),
            format!("%{}%", query)
        )
        .fetch_all(&*pool)
        .await
        .unwrap_or_default()
    };

    let template = DocumentsTemplate {
        documents,
        query,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Template error")))
}

#[debug_handler]
async fn search_page(
    Query(params): Query<SearchParams>,
    State(pool): State<Arc<SqlitePool>>,
) -> Html<String> {
    let query = params.q.unwrap_or_default();
    
    let results = if !query.is_empty() {
        sqlx::query_as!(
            crate::models::SearchResult,
            r#"
            SELECT sr.* 
            FROM search_results sr
            JOIN search_queries sq ON sr.query_id = sq.id
            WHERE sq.query = ?
            ORDER BY sr.relevance_score DESC
            LIMIT 20
            "#,
            query
        )
        .fetch_all(&*pool)
        .await
        .unwrap_or_default()
    } else {
        Vec::new()
    };

    let template = SearchTemplate {
        results,
        query,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Template error")))
}

#[debug_handler]
async fn view_logs(
    Query(params): Query<SearchParams>,
    State(pool): State<Arc<SqlitePool>>,
) -> Html<String> {
    let query = params.q.unwrap_or_default();
    let level = params.level.unwrap_or_default();
    
    let logs = if query.is_empty() && level.is_empty() {
        crate::logging::get_recent_logs(&pool, 100).await.unwrap_or_default()
    } else {
        let mut sql = String::from(
            "SELECT level, message, created_at FROM application_logs WHERE 1=1"
        );
        let mut params = Vec::new();
        
        if !query.is_empty() {
            sql.push_str(" AND message LIKE ?");
            params.push(format!("%{}%", query));
        }
        
        if !level.is_empty() {
            sql.push_str(" AND level = ?");
            params.push(level.clone());
        }
        
        sql.push_str(" ORDER BY created_at DESC LIMIT 100");
        
        sqlx::query(&sql)
            .bind(&params[0])
            .bind(&params.get(1).unwrap_or(&String::new()))
            .map(|row| (
                row.get(0),
                row.get(1),
                row.get(2)
            ))
            .fetch_all(&*pool)
            .await
            .unwrap_or_default()
    };

    let template = LogsTemplate {
        logs,
        query,
        level,
    };
    
    Html(template.render().unwrap_or_else(|_| String::from("Template error")))
}
