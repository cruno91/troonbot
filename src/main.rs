mod config;
mod fetch;
mod models;
mod twilio;

use tokio::time::{sleep, Duration};
use fetch::fetch_products;
use twilio::send_sms;

#[tokio::main]
async fn main() {
    let cfg = config::load();

    let mut current_beer = String::new();

    loop {
        if let Ok(products) = fetch_products().await {
            if let Some(first_beer) = products.first() {
                if first_beer.name != current_beer {
                    current_beer = first_beer.name.clone();
                    let body = format!(
                        "Name: {}\nPrice: {}",
                        first_beer.name, first_beer.price.low_formatted_with_modifiers
                    );
                    send_sms(&cfg, &body).await;
                }
            }
        }

        sleep(Duration::from_secs(300)).await;
    }
}
