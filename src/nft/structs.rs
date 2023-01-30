#![allow(dead_code)]
pub struct NftInfo {
    name: String,
    symbol: String,
    description: String,
    seller_fee_basis_points: u32,
    external_url: String,
    edition: String,
    background_color: String,
    attributes: Vec<Attribute>,
    properties: TokenProperties,
    image: String,
}

pub struct Attribute {
    trait_type: String,
    value: String,
    display_type: Option<String>,
}

pub struct TokenProperties {
    category: String,
    creators: Vec<Creator>,
    files: Vec<File>,
}

pub struct Creator {
    address: String,
    share: u8,
}

pub struct File {
    uri: String,
    r#type: String,
}
