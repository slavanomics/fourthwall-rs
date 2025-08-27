use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Money { pub value: f64, pub currency: String }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image { pub id: String, pub url: String, pub width: i32, pub height: i32, pub transformedUrl: String }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Variant {
    pub id: String,
    pub name: String,
    pub sku: String,
    pub unitPrice: Money,
    pub compareAtPrice: Option<Money>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub images: Vec<Image>,
    pub variants: Vec<Variant>,
}
