mod config;
mod discord;
mod fetch;
mod models;

use tokio::time::{sleep, Duration};
use fetch::fetch_products;
use discord::send_discord_message;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let cfg = config::load();

    // Store previous beers and their URLs
    let mut previous_beers: Vec<String> = Vec::new();
    let mut previous_beers_url: HashMap<String, String> = HashMap::new();

    loop {
        if let Ok(products) = fetch_products().await {
            for product in &products {
                let product_name = &product.name;

                // Check if this is a new beer
                if !previous_beers.contains(product_name) {
                    previous_beers.push(product_name.clone());
                    previous_beers_url.insert(product_name.clone(), product.absolute_site_link.clone());

                    // Send notification for new listing
                    let content = format!(
                        "<@&{}> {} was just listed. (For sale probably later today.)",
                        cfg.discord_listing_role_id, product_name
                    );
                    send_discord_message(&cfg, &content).await;
                } else {
                    // Check if the URL changed from a "filler" URL to a real one
                    if let Some(prev_url) = previous_beers_url.get(product_name) {
                        if prev_url.contains("filler") && !product.absolute_site_link.contains("filler") {
                            // Update the URL
                            previous_beers_url.insert(product_name.clone(), product.absolute_site_link.clone());

                            // Send notification that the beer is now for sale
                            let content = format!(
                                "<@&{}> {} is now for sale! {}",
                                cfg.discord_sale_role_id, product_name, product.absolute_site_link
                            );
                            send_discord_message(&cfg, &content).await;
                        }
                    }
                }
            }
        }

        // Sleep for 1 minute before checking again
        sleep(Duration::from_secs(60)).await;
    }
}