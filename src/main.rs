use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

mod routes;
use crate::routes::users::create_users;

// Check tower crate, 
// tokio blog on serve trait

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_users));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let (shutdown, rx) = tokio::sync::oneshot::channel::<()>();
    // shutdown.send(());
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async move {
            let _ = rx.await;
            //println!("graceful shutdown");
        })
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}