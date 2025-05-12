# TroonBot

_Rust implementation of [Cobra800089 / troonChecker](https://github.com/Cobra800089/troonChecker) for fun._

**TroonBot** is a Rust-based CLI tool that monitors product listings and sends Discord notifications when new items 
appear or go on sale. 
---

## 📦 Features

- Periodically fetches product data from a remote source
- Sends Discord webhook messages when:
    - A new product is listed
    - A previously listed product becomes available for sale
- Supports `.env`, OS environment variables, and CLI argument overrides
- Lightweight and runs on Tokio async runtime

---

## ⚙️ Configuration

You can configure the bot in **three ways**:

### 1. `.env` file (optional)

Create a `.env` file in the root of your project:

```env
DISCORD_WEBHOOK_URL="your_discord_webhook_url"
DISCORD_USERNAME="TroonBot"
DISCORD_LISTING_ROLE_ID="your_listing_role_id"
DISCORD_SALE_ROLE_ID="your_sale_role_id"
```

### 2. Environment Variables

Set variables manually before running:

```bash
export DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/..."
export DISCORD_LISTING_ROLE_ID="123456"
```

### 3. Command-line Arguments

You can override any of the above via flags:

```bash
./troonbot \
  --discord-webhook-url https://discord.com/api/webhooks/... \
  --discord-username TroonBot \
  --discord-listing-role-id 123456 \
  --discord-sale-role-id 654321
```

Short flags are also supported:

```bash
./troonbot -w https://discord.com/api/webhooks/... -l 123456 -s 654321
```

---

## 🛠 CLI Options

```bash
./troonbot --help
```

```
Usage: troonbot [OPTIONS]

Options:
  -w, --discord-webhook-url <URL>     Discord webhook URL
  -u, --discord-username <USERNAME>   Discord bot username [default: TroonBot]
  -l, --discord-listing-role-id <ID>  Role ID to ping when new products appear
  -s, --discord-sale-role-id <ID>     Role ID to ping when products go on sale
  -h, --help                           Show help
  -V, --version                        Show version
```
---

## 🚀 Development

### Prerequisites

- Rust and Cargo: [https://rustup.rs](https://rustup.rs)
- A Discord webhook URL
- Discord role IDs (2) for new listings and new sale items

### Build the Project

```bash
cargo build --release
```

This creates the executable at:

```
./target/release/troonbot
```
