use cache::PostgresCache;
use log::info;
use server_counter::{http::Server, Config, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    info!("Loading Config...");
    let config = Config::new();
    info!(config.cache_uri);

    info!("Connecting to Cache...");
    let cache = PostgresCache::connect(config.cache_uri.clone(), cache::Options::default(), 1)
        .await
        .map_err(Error::CacheError)?;

    let server = Server::new(config, cache);
    info!("Starting server...");
    server.start().await
}
