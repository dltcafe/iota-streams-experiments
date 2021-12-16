use anyhow::Result;
use std::process;

use iota_streams_experiments::config::Config;
use iota_streams_experiments::examples;
use iota_streams_experiments::utilities::header;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().unwrap_or_else(|err| {
        eprintln!("Problem reading arguments: {:?}", err);
        process::exit(1);
    });

    header("Beginning execution", 0, 0);
    println!("{:#?}", config);

    header("Publish publisher", 1, 0);
    examples::public_publisher::execute(&config).await?;

    Ok(())
}
