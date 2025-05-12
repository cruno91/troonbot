use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TroonData {
    pub data: Vec<Product>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Product {
    pub name: String,
    pub absolute_site_link: String,
    // pub price: Price,
    // pub badges: Badges,
}

// #[derive(Debug, Deserialize, Clone)]
// pub struct Price {
//     pub low_formatted_with_modifiers: String,
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct Badges {
//     pub on_sale: bool,
// }