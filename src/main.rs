use axum::{extract::Path, routing::get};
use tokio::net::TcpListener;

async fn double(Path(input): Path<String>) -> String {
    match input.parse::<i32>() {
        Ok(num) => format!("{} times 2 is {}!", num, num * 2),
        Err(e) => format!("Uh oh, weird input: {e}")
    }
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(|| async { "The server works!" }))
        .route("/game/:guess",
               get(|Path(guess): Path<String>| async move { format!("The guess is {guess}") }),
        )
        .route("/double/:number", get(double));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
