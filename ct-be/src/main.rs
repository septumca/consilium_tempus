use axum::{
    http::Method,
    routing::{get, post, delete, put},
    Router, middleware,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use std::{net::SocketAddr};
use tokio::signal;

mod utils;
mod auth;
mod users;
mod tasks;
mod reference_data;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers(Any)
        .allow_origin(Any);

    let public = Router::new()
        .route("/register", post(users::create))
        .route("/authentificate", post(users::authentificate));


    let private = Router::new()
        // .route("/users", post(utils::create::<users::CreateUserData, users::User>))
        .route("/users", get(utils::read_all::<users::User>))
        .route("/users/:id", get(utils::read::<users::User>))
        .route("/users/:id", put(utils::put::<users::UpdateUserData, users::User>))
        .route("/users/:id", delete(utils::delete::<users::User>))

        .route("/tasks", post(utils::create::<tasks::CreateTaskData, tasks::Task>))
        .route("/tasks", get(utils::read_all::<tasks::Task>))
        .route("/tasks/user/:id", get(tasks::read_users_tasks))
        .route("/tasks/:id", get(utils::read::<tasks::Task>))
        .route("/tasks/:id", put(utils::put::<tasks::UpdateTaskData, tasks::Task>))
        .route("/tasks/:id", delete(utils::delete::<tasks::Task>))

        .route("/reference_data", get(reference_data::read))
        .route_layer(middleware::from_fn(auth::auth));

    let app = Router::new()
        .merge(public)
        .merge(private)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7005));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}