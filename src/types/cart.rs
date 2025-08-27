use serde::{Deserialize, Serialize};
use super::{Image, Money, Product};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cart {
    pub id: String,
    pub items: Vec<CartItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CartItem {
    pub variant: CartVariant,
    pub quantity: i32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CartVariant {
    pub id: String,
    pub name: String,
    pub sku: String,
    pub unitPrice: Money,
    pub compareAtPrice: Option<Money>,
    pub images: Vec<Image>,
    pub product: ProductStub,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductStub { pub id: String, pub name: String, pub slug: String }

#[derive(Debug, Clone, Serialize)]
pub struct CreateCartBody {
    pub items: Vec<RequestItem>,
}
#[derive(Debug, Clone, Serialize)]
pub struct RequestItem {
    pub variantId: String,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct MutateCartBody {
    pub items: Vec<MutateItem>,
}
#[derive(Debug, Clone, Serialize)]
pub struct MutateItem {
    pub variantId: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>, // change uses quantity; remove ignores it
}
