use miniserve::{Content, Request, Response};
use serde_json::json;

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(_req: Request) -> Response {
    let response = json!({
        "messages": [
            "Hello world",
            "And how does that make you feel?"
        ]
    });
    let response_string = response.to_string();
    Ok(Content::Json(response_string))
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}
