use askama::Template;
use axum::{Router, extract::State, http::StatusCode, response::Html, routing::get};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    entries: &'a [Entry],
}

#[derive(FromRow)]
struct Entry {
    username: String,
    score: i64,
}

async fn index(State(pool): State<PgPool>) -> Result<Html<String>, StatusCode> {
    let entries = sqlx::query_as::<_, Entry>(
        "SELECT username, score FROM users ORDER BY score DESC, username LIMIT 10",
    )
    .fetch_all(&pool)
    .await
    .map_err(|error| {
        eprintln!("failed to load leaderboard: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    IndexTemplate { entries: &entries }
        .render()
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("database should connect");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("database migrations should run");

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("address should bind");

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.expect("server should run");
}
