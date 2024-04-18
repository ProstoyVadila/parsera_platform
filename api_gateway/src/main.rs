

mod config;
mod app;
mod routes;
mod api;
mod database;
mod rabbit;
mod utils;
mod redis;


#[tokio::main]
async fn main() {
    let cfg = config::Config::new();
    cfg.set_tracing();

    tracing::info!("Starting api gateway service...");
    let app = app::App::new(cfg).await;    // run it with hyper

    tracing::info!("listening on {}", app.config.get_socket_addr());
    let listener = tokio::net::TcpListener::bind(app.config.get_socket_addr())
        .await
        .unwrap();
    axum::serve(listener, app.router)
        .with_graceful_shutdown(utils::graceful_shutdown())
        .await.unwrap();

}
