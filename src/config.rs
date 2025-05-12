use clap::Parser;
use std::env;
use dotenvy::dotenv;

/// Command-line arguments override env vars or .env
#[derive(Parser, Debug)]
#[command(
    name = "TroonBot",
    version,
    author,
    about = "Watches product listings and posts updates to Discord",
    long_about = "Use environment variables, .env, or pass arguments to the CLi to configure"
)]
struct Args {
    #[arg(short = 'w', long = "discord-webhook-url")]
    discord_webhook_url: Option<String>,

    #[arg(short = 'u', long = "discord-username")]
    discord_username: Option<String>,

    #[arg(short = 'l', long = "discord-listing-role-id")]
    discord_listing_role_id: Option<String>,

    #[arg(short = 's', long = "discord-sale-role-id")]
    discord_sale_role_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub discord_webhook_url: String,
    pub discord_username: String,
    pub discord_listing_role_id: String,
    pub discord_sale_role_id: String,
}

impl Config {
    pub fn load() -> Self {
        // Load .env if it exists
        dotenv().ok();

        let args = Args::parse();

        Config {
            discord_webhook_url: args.discord_webhook_url
                .or_else(|| env::var("DISCORD_WEBHOOK_URL").ok())
                .expect("Missing DISCORD_WEBHOOK_URL"),

            discord_username: args.discord_username
                .or_else(|| env::var("DISCORD_USERNAME").ok())
                .unwrap_or_else(|| "TroonBot".to_string()),

            discord_listing_role_id: args.discord_listing_role_id
                .or_else(|| env::var("DISCORD_LISTING_ROLE_ID").ok())
                .expect("Missing DISCORD_LISTING_ROLE_ID"),

            discord_sale_role_id: args.discord_sale_role_id
                .or_else(|| env::var("DISCORD_SALE_ROLE_ID").ok())
                .expect("Missing DISCORD_SALE_ROLE_ID"),
        }
    }
}
