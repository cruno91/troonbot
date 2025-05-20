use crate::config::Config;
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct DiscordMessage {
    username: String,
    content: String,
}

pub async fn send_discord_message(config: &Config, content: String) {
    let client = Client::new();

    let message = DiscordMessage {
        username: config.discord_username.clone(),
        content: content.to_string(),
    };

    let res = client
        .post(&config.discord_webhook_url)
        .json(&message)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("Discord message sent successfully!");
            } else {
                println!("Failed to send Discord message: {:?}", response.text().await);
            }
        }
        Err(err) => eprintln!("Error sending Discord message: {:?}", err),
    }
}
