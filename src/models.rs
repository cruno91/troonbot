use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TroonData {
    pub data: Vec<Product>,
}

#[derive(Debug, Deserialize)]
pub struct Product {
    pub name: String,
    // pub site_link: String,
    pub price: Price,
}

#[derive(Debug, Deserialize)]
pub struct Price {
    pub low_formatted_with_modifiers: String,
}