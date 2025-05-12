use std::env;
use dotenv::dotenv;

pub struct Config {
    pub discord_webhook_url: String,
    pub discord_username: String,
    pub discord_listing_role_id: String,
    pub discord_sale_role_id: String,
}

pub fn load() -> Config {
    dotenv().ok();

    Config {
        discord_webhook_url: env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL not set"),
        discord_username: env::var("DISCORD_USERNAME").unwrap_or_else(|_| "TroonBot".to_string()),
        discord_listing_role_id: env::var("DISCORD_LISTING_ROLE_ID").expect("DISCORD_LISTING_ROLE_ID not set"),
        discord_sale_role_id: env::var("DISCORD_SALE_ROLE_ID").expect("DISCORD_SALE_ROLE_ID not set"),
    }
}