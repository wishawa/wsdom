mod audio;
mod canvas;
mod computation;
mod counter;

use std::future::Future;

use axum::{response::Html, routing::get, Router};
use wsdom::Browser;
use wsdom_axum::browser_handler;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .nest("/audio", make_one_demo("audio", audio::app))
        .nest("/canvas", make_one_demo("canvas", canvas::app))
        .nest(
            "/computation",
            make_one_demo("computation", computation::app),
        )
        .nest("/counter", make_one_demo("counter", counter::app));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

fn make_one_demo<Func, Fut>(name: &str, func: Func) -> Router
where
    Func: Clone + Send + 'static + Fn(Browser) -> Fut,
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    let router = Router::new()
        .route("/", get(make_html(&*format!("{name}/ws"))))
        .route("/ws", get(browser_handler(func)));
    router
}

const HTML_TEMPLATE: &'static str = include_str!("../index.html");
fn make_html(ws_path: &str) -> Html<String> {
    Html(HTML_TEMPLATE.replace(
        "\"ws://localhost:4000/ws\"",
        &format!("\"ws://localhost:4000/{ws_path}\""),
    ))
}
