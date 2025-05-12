use std::env;
use dotenv::dotenv;

pub struct Config {
    pub to_phone: String,
    pub from_phone: String,
    pub sid: String,
    pub auth_token: String,
}

pub fn load() -> Config {
    dotenv().ok();

    Config {
        to_phone: env::var("TO_PHONE").expect("TO_PHONE not set"),
        from_phone: env::var("TWILIO_PHONE").expect("TWILIO_PHONE not set"),
        sid: env::var("TWILIO_SID").expect("TWILIO_SID not set"),
        auth_token: env::var("TWILIO_AUTH").expect("TWILIO_AUTH not set"),
    }
}
