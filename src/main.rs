use askama::Template;
use axum::{Router, response::Html, routing::get};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> Html<String> {
    Html(IndexTemplate.render().expect("template should render"))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("address should bind");

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.expect("server should run");
}
