use anyhow::Result;
use iota_streams::app_channels::api::tangle::{MessageContent, UnwrappedMessage};
use rand::Rng;

pub const ALPH_9: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";

pub fn new_seed() -> String {
    (0..10)
        .map(|_| {
            ALPH_9
                .chars()
                .nth(rand::thread_rng().gen_range(0, 27))
                .unwrap()
        })
        .collect::<String>()
}

pub fn header(msg: &str, spaces_prev: usize, spaces_after: usize) {
    println!(
        "{}{}\n# {} #\n{}{}",
        "\n".repeat(spaces_prev),
        "#".repeat(msg.len() + 4),
        msg,
        "#".repeat(msg.len() + 4),
        "\n".repeat(spaces_after)
    );
}

/// Copied from https://github.com/iotaledger/streams-examples/blob/master/src/examples/mod.rs
pub fn verify_messages(
    sent_messages: &[&str],
    retrieved_messages: Vec<UnwrappedMessage>,
) -> Result<()> {
    let processed_messages = retrieved_messages
        .iter()
        .map(|msg| {
            let content = &msg.body;
            match content {
                MessageContent::SignedPacket {
                    pk: _,
                    public_payload: _,
                    masked_payload,
                } => String::from_utf8(masked_payload.0.to_vec()).unwrap(),
                _ => String::default(),
            }
        })
        .filter(|s| s != &String::default())
        .collect::<Vec<String>>();

    if processed_messages.is_empty() && sent_messages.is_empty() {
        return Ok(());
    }

    println!("Retrieved messages: ");
    for i in 0..processed_messages.len() {
        println!("{}", processed_messages[i]);
        assert_eq!(processed_messages[i], sent_messages[i])
    }

    Ok(())
}
