mod config;
use config::Config;

mod client;
use client::Client;

mod error;
pub use error::{AppError, Result};

use log::{debug, error};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    sleep(Duration::from_secs(10));

    let conf = Config::load();
    let client = Client::new(conf);

    loop {
        do_update(&client);
        sleep(Duration::from_secs(60 * 5));
    }
}

fn do_update(client: &Client) {
    let count = match client.fetch_server_count() {
        Ok(v) => v,
        Err(e) => {
            error!("Error fetching server count: {}", e);
            return;
        }
    };

    debug!("Server anzahl: {}", count);

    if let Err(e) = client.update_channel(count) {
        error!("Error updating channel: {}", e);
        return;
    }

    debug!("Updated channel successfully");
}
