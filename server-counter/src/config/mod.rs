use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_server_addr")]
    pub server_addr: String,

    #[serde(default = "default_cache_uri")]
    pub cache_uri: String,
}

impl Config {
    pub fn new() -> Config {
        envy::from_env().expect("failed to parse config")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

// Default value providers
fn default_server_addr() -> String {
    "127.0.0.1:8080".to_string()
}

fn default_cache_uri() -> String {
    "postgresql://postgres:null@postgres-cache:5432/botcache".to_string()
}
