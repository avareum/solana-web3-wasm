use solana_client_wasm::WasmClient;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};

use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Debug)]
pub enum EndPoint {
    #[strum(serialize = "https://api.mainnet-beta.solana.com")]
    Mainnet,
    #[strum(serialize = "https://api.devnet.solana.com")]
    Devnet,
}

pub struct Web3Provider {
    pub client: WasmClient,
}

impl Web3Provider {
    pub fn new(endpoint: &EndPoint) -> Self {
        Self::new_with_client(WasmClient::new_with_commitment(
            endpoint.to_string().as_ref(),
            CommitmentConfig {
                commitment: CommitmentLevel::Finalized,
            },
        ))
    }

    pub fn new_with_client(client: WasmClient) -> Web3Provider {
        Web3Provider { client }
    }

    pub fn new_mainnet() -> Web3Provider {
        Web3Provider::new(&EndPoint::Mainnet)
    }

    #[allow(dead_code)]
    pub fn new_devnet() -> Web3Provider {
        Web3Provider::new(&EndPoint::Devnet)
    }
}
