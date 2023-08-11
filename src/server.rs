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
    http::{header::CONTENT_TYPE, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
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
        .layer(middleware::from_fn(convert_content_type))
        .with_state(Arc::new(state));

    let listen_address: SocketAddr = args.listen_address().parse()?;

    tracing::info!("Listening on {listen_address}");

    axum::Server::bind(&listen_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn convert_content_type<B>(mut request: Request<B>, next: Next<B>) -> Response {
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
