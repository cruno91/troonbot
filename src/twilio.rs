use crate::config::Config;
use reqwest::Client;

pub async fn send_sms(config: &Config, body: &str) {
    let url = format!(
        "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
        config.sid
    );

    let client = Client::new();
    let params = [
        ("To", config.to_phone.as_str()),
        ("From", config.from_phone.as_str()),
        ("Body", body),
    ];

    let res = client
        .post(&url)
        .basic_auth(&config.sid, Some(&config.auth_token))
        .form(&params)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("SMS sent successfully!");
            } else {
                println!("Failed to send SMS: {:?}", response.text().await);
            }
        }
        Err(err) => eprintln!("Error sending SMS: {:?}", err),
    }
}
