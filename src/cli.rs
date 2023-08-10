use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    hostname: String,
    #[arg(long, default_value_t = 3000)]
    port: u16,
    #[arg(long)]
    mongodb_url: String,
}

impl Cli {
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }

    pub fn mongodb_url(&self) -> &str {
        &self.mongodb_url
    }
}
