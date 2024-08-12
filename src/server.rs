mod aggregate;
mod delete_many;
mod delete_one;
mod drop_database;
mod find_many;
mod find_one;
mod insert_many;
mod insert_one;
mod update_many;
mod update_one;

mod normalize;
mod state;

pub use state::AppState;

use crate::cli::Cli;
use axum::{
    extract::Request,
    http::{header::CONTENT_TYPE, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;

pub async fn start(args: &Cli) -> anyhow::Result<()> {
    let state = AppState::new(args).await?;

    let app = Router::new()
        .route(&atlas_route("findOne"), post(find_one::handler))
        .route(&atlas_route("find"), post(find_many::handler))
        .route(&atlas_route("insertOne"), post(insert_one::handler))
        .route(&atlas_route("insertMany"), post(insert_many::handler))
        .route(&atlas_route("updateOne"), post(update_one::handler))
        .route(&atlas_route("updateMany"), post(update_many::handler))
        .route(&atlas_route("deleteOne"), post(delete_one::handler))
        .route(&atlas_route("deleteMany"), post(delete_many::handler))
        .route(&atlas_route("aggregate"), post(aggregate::handler))
        .route(&atlas_route("dropDatabase"), post(drop_database::handler))
        .layer(middleware::from_fn(convert_content_type))
        .with_state(Arc::new(state));

    let listen_address: SocketAddr = args.listen_address().parse()?;
    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();

    tracing::info!("Listening on {listen_address}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn convert_content_type(mut request: Request, next: Next) -> Response {
    match request
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
    {
        Some("application/ejson") | Some("application/json") => (),
        _ => {
            let response = (
                StatusCode::BAD_REQUEST,
                "Only application/ejson and application/json content types accepted.",
            );

            return response.into_response();
        }
    }

    request
        .headers_mut()
        .insert(CONTENT_TYPE, "application/json".parse().unwrap());

    next.run(request).await
}

fn atlas_route(action: &str) -> String {
    format!("/app/data-test/endpoint/data/v1/action/{action}")
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
    println!("Shutting down gracefully...");
}
