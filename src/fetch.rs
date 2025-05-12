use crate::models::{Product, TroonData};
use reqwest::Client;

pub async fn fetch_products() -> Result<Vec<Product>, reqwest::Error> {
    let url = "https://cdn5.editmysite.com/app/store/api/v16/editor/users/131270493/sites/827516815791883917/products";
    let client = Client::new();

    let res = client
        .get(url)
        .header("User-Agent", "troonChecker")
        .send()
        .await?
        .json::<TroonData>()
        .await?;

    Ok(res.data)
}
