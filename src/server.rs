mod find_many;
mod find_one;
mod normalize;
mod state;

pub use state::AppState;

use crate::cli::Cli;
use axum::{routing::post, Router};
use std::{net::SocketAddr, sync::Arc};

pub async fn start(args: &Cli) -> anyhow::Result<()> {
    let state = AppState::new(args).await?;

    let app = Router::new()
        .route(&atlas_route("findOne"), post(find_one::handler))
        .route(&atlas_route("find"), post(find_many::handler))
        .with_state(Arc::new(state));

    let listen_address: SocketAddr = args.listen_address().parse()?;

    tracing::info!("Listening on {listen_address}");

    axum::Server::bind(&listen_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn atlas_route(action: &str) -> String {
    format!("/app/data-test/endpoint/data/v1/action/{action}")
}
