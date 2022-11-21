use reqwest::Url;
use solana_client_wasm::WasmClient;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};

use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Debug)]
pub enum EndPoint {
    #[strum(serialize = "https://api.mainnet-beta.solana.com")]
    Mainnet,
    #[strum(serialize = "https://api.devnet.solana.com")]
    Devnet,
    #[strum(disabled)]
    CustomUrl(Url),
}

pub struct Web3Provider {
    pub client: WasmClient,
}

impl Web3Provider {
    pub fn new(endpoint: &EndPoint) -> Self {
        Web3Provider::new_with_client(WasmClient::new_with_commitment(
            match endpoint {
                EndPoint::Mainnet => EndPoint::Mainnet.to_string(),
                EndPoint::Devnet => EndPoint::Devnet.to_string(),
                EndPoint::CustomUrl(url) => url.to_string(),
            }
            .as_ref(),
            CommitmentConfig {
                commitment: CommitmentLevel::Finalized,
            },
        ))
    }

    pub fn new_with_client(client: WasmClient) -> Web3Provider {
        Self { client }
    }

    pub fn new_mainnet() -> Web3Provider {
        Web3Provider::new(&EndPoint::Mainnet)
    }

    #[allow(dead_code)]
    pub fn new_devnet() -> Web3Provider {
        Web3Provider::new(&EndPoint::Devnet)
    }
}

#[cfg(test)]
mod test {

    use reqwest::Url;

    use super::{EndPoint, Web3Provider};

    #[tokio::test]
    async fn test_new() {
        let provider = Web3Provider::new_mainnet();
        assert!(provider.client.get_latest_blockhash().await.is_ok());

        let provider = Web3Provider::new_devnet();
        assert!(provider.client.get_latest_blockhash().await.is_ok());

        let provider = Web3Provider::new(&EndPoint::CustomUrl(
            Url::parse("https://api.mainnet-beta.solana.com").unwrap(),
        ));
        assert!(provider.client.get_latest_blockhash().await.is_ok());
    }
}
