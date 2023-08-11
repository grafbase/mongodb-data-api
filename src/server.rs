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
use axum::{routing::post, Router};
use std::{net::SocketAddr, sync::Arc};

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
