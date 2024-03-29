use solana_client_wasm::WasmClient;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};

use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Debug)]
pub enum EndPoint {
    #[strum(serialize = "https://api.mainnet-beta.solana.com")]
    Mainnet,
    #[strum(serialize = "https://api.devnet.solana.com")]
    Devnet,
    #[strum(serialize = "https://api.testnet.solana.com")]
    Testnet,
    #[strum(disabled)]
    CustomUrl(String),
}

#[derive(EnumString, Display, Debug)]
pub enum ClusterId {
    #[strum(serialize = "mainnet-beta")]
    Mainnet,
    #[strum(serialize = "devnet")]
    Devnet,
    #[strum(serialize = "testnet")]
    Testnet,
}

pub trait Web3WasmClient {
    fn new(endpoint: &EndPoint) -> Self;
    fn new_mainnet() -> Self;
    fn new_devnet() -> Self;
    fn new_testnet() -> Self;
}

impl Web3WasmClient for WasmClient {
    fn new(endpoint: &EndPoint) -> Self {
        let endpoint = match endpoint {
            EndPoint::Mainnet => EndPoint::Mainnet.to_string(),
            EndPoint::Devnet => EndPoint::Devnet.to_string(),
            EndPoint::Testnet => EndPoint::Testnet.to_string(),
            EndPoint::CustomUrl(url) => url.to_string(),
        };

        WasmClient::new_with_commitment(
            endpoint.as_ref(),
            CommitmentConfig {
                commitment: CommitmentLevel::Confirmed,
            },
        )
    }

    fn new_mainnet() -> Self {
        Web3WasmClient::new(&EndPoint::Mainnet)
    }

    fn new_devnet() -> Self {
        Web3WasmClient::new(&EndPoint::Devnet)
    }

    fn new_testnet() -> Self {
        Web3WasmClient::new(&EndPoint::Testnet)
    }
}
