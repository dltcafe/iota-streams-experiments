use crate::config::Config;
use crate::utilities::verify_messages;
use iota_streams::app::transport::tangle::client::Client;
use iota_streams::app::transport::tangle::TangleAddress;
use iota_streams::app_channels::api::tangle::{Address, Author, Subscriber};
use iota_streams::app_channels::api::ChannelType;
use iota_streams::core::Result;
use iota_streams::ddml::types::Bytes;

pub async fn execute(config: &Config) -> Result<()> {
    let messages = vec!["https://dlt.cafe", "dlt", "cafe", "working", "group"];
    let (author, announcement_address) = create_channel(config).await?;
    publish_messages(config, &messages, &announcement_address, author).await?;
    receive_messages(config, &messages, &announcement_address).await?;

    Ok(())
}

async fn create_channel(config: &Config) -> Result<(Author<Client>, Address)> {
    let mut author = Author::new(
        config.seed(),
        ChannelType::SingleBranch,
        config.client().clone(),
    );

    let announcement_message_address = author.send_announce().await?;
    display_message_info(&announcement_message_address, config.explorer());

    Ok((author, announcement_message_address))
}

fn display_message_info(message_address: &TangleAddress, explorer: &str) {
    println!("Sent message: {}", message_address);
    println!(
        "Tangle Index: {}{:#}",
        explorer,
        message_address.to_msg_index()
    );
}

async fn publish_messages(
    config: &Config,
    messages: &[&str],
    announcement_address: &TangleAddress,
    mut author: Author<Client>,
) -> Result<()> {
    let mut previous_message_address = *announcement_address;
    for message in messages {
        let (message_address, _) = author
            .send_signed_packet(
                &previous_message_address,
                &Bytes::default(),
                &Bytes(message.as_bytes().to_vec()),
            )
            .await?;
        display_message_info(&message_address, config.explorer());
        previous_message_address = message_address;
    }

    Ok(())
}

async fn receive_messages(
    config: &Config,
    messages: &[&str],
    announcement_address: &TangleAddress,
) -> Result<()> {
    let mut subscriber = Subscriber::new("Subscriber", config.client().clone());
    subscriber
        .receive_announcement(announcement_address)
        .await?;
    let retrieved = subscriber.fetch_all_next_msgs().await;
    verify_messages(messages, retrieved)?;

    Ok(())
}
