use anyhow::Result;

use crate::utilities::new_seed;
use dotenv::dotenv;
use iota_streams::app::transport::tangle::client::Client;
use std::env;
use std::fmt::{Debug, Formatter};

pub const NODE: &str = "NODE";
pub const SEED: &str = "SEED";
pub const EXPLORER: &str = "EXPLORER";

pub struct Config {
    node: String,
    explorer: String,
    seed: String,
    client: Client,
}

impl Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.node)
            .field(&self.explorer)
            .field(&self.seed)
            .finish()
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok();

        let node = env::var(NODE)?;
        let explorer = env::var(EXPLORER).unwrap_or_else(|_| String::from(""));
        let seed = env::var(SEED).unwrap_or_else(|_| new_seed());
        let client = Client::new_from_url(node.as_str());

        Ok(Self {
            node,
            explorer,
            seed,
            client,
        })
    }

    pub fn node(&self) -> &str {
        &self.node
    }

    pub fn seed(&self) -> &str {
        &self.seed
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn explorer(&self) -> &str {
        &self.explorer
    }
}
