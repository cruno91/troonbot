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

    use futures::future::join_all;

    loop {
        if let Ok(products) = fetch_products().await {
            let mut tasks = Vec::new();

            for product in &products {
                let product_name = &product.name;

                // New beer listing
                if !previous_beers.contains(product_name) {
                    previous_beers.push(product_name.clone());
                    previous_beers_url.insert(product_name.clone(), product.absolute_site_link.clone());

                    let content = format!(
                        "<@&{}> {} was just listed. (For sale probably later today.)",
                        cfg.discord_listing_role_id, product_name
                    );

                    // Push to futures (concurrent) task list
                    tasks.push(send_discord_message(&cfg, content));
                } else if let Some(prev_url) = previous_beers_url.get(product_name) {
                    // Sale status update
                    if prev_url.contains("filler") && !product.absolute_site_link.contains("filler") {
                        previous_beers_url.insert(product_name.clone(), product.absolute_site_link.clone());

                        let content = format!(
                            "<@&{}> {} is now for sale! {}",
                            cfg.discord_sale_role_id, product_name, product.absolute_site_link
                        );

                        // Push to futures (concurrent) task list
                        tasks.push(send_discord_message(&cfg, content));
                    }
                }
            }

            // Await all notifications concurrently so messages do not block
            // each other during sending.
            join_all(tasks).await;
        }

        sleep(Duration::from_secs(60)).await;
    }

}