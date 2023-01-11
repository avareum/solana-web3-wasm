pub struct Nft {
    name: String,
    symbol: String,
    description: String,
    sellerFeeBasisPoints: u32,
    externalUrl: String,
    edition: String,
    backgroundColor: String,
    attributes: Vec<Attribute>,
    properties: TokenProperties,
    image: String,
}

pub struct Attribute {
    traitType: String,
    value: String,
    displayType: Option<String>,
}

pub struct NftProperties {
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
