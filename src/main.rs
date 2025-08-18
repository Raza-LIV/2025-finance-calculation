mod handlers;
mod routes;

fn app() -> axum::Router {
    routes::create_router()
}

#[cfg(not(feature = "lambda-deploy"))]
#[tokio::main]
async fn main() {
    use std::net::SocketAddr;
    
    tracing_subscriber::fmt().json().init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

#[cfg(feature = "lambda-deploy")]
#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    lambda_http::run(app()).await
}
