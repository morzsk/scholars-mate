use askama::Template;
use axum::{Router, http::StatusCode, response::Html, routing::get};
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    entries: &'a [Entry],
}

struct Entry {
    username: &'static str,
    score: u32,
}

const ENTRIES: &[Entry] = &[
    Entry {
        username: "AAA",
        score: 10_000,
    },
    Entry {
        username: "BBB",
        score: 9_000,
    },
    Entry {
        username: "CCC",
        score: 8_000,
    },
    Entry {
        username: "DDD",
        score: 7_000,
    },
    Entry {
        username: "EEE",
        score: 6_000,
    },
    Entry {
        username: "FFF",
        score: 5_000,
    },
    Entry {
        username: "GGG",
        score: 4_000,
    },
    Entry {
        username: "HHH",
        score: 3_000,
    },
    Entry {
        username: "III",
        score: 2_000,
    },
    Entry {
        username: "JJJ",
        score: 1_000,
    },
];

async fn index() -> Result<Html<String>, StatusCode> {
    IndexTemplate { entries: ENTRIES }
        .render()
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("address should bind");

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.expect("server should run");
}
