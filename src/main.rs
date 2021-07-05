use warp::Filter;

#[tokio::main]
async fn main() {
    let hi = warp::path!("hi")
        .and(warp::get())
        .map(|| "Hello world!!");
    let message = warp::path!("hi")
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: models::Message| format!("Hello, {message}!!", message=body.message));
    let routes = warp::path("api")
            .and(warp::path("v1")
                .and(hi.or(message)));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Message {
        pub message: String
    }
}