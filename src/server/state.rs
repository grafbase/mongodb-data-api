use mongodb::{options::ClientOptions, Client};

use crate::cli::Cli;

#[derive(Debug)]
pub struct AppState {
    mongo: Client,
}

impl AppState {
    pub async fn new(args: &Cli) -> anyhow::Result<Self> {
        let mongo = Client::with_options(ClientOptions::parse(args.mongodb_url()).await?)?;

        Ok(Self { mongo })
    }

    pub fn mongo(&self) -> &Client {
        &self.mongo
    }
}
