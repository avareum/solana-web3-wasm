use solana_client_wasm::WasmClient;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};

use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Debug)]
pub enum EndPoint {
    #[strum(serialize = "https://rpc.ankr.com/solana")]
    Mainnet,
    #[strum(serialize = "https://api.devnet.solana.com")]
    Devnet,
    // This throw "Uncaught (in promise) LinkError: WebAssembly.instantiate(): Import #80 module="wbg" function="__wbindgen_closure_wrapper2087" error: function import requires a callable"
    // #[strum(disabled)]
    // CustomUrl(String),
}

// pub struct Web3WasmClient {
//     pub client: WasmClient,
// }

pub trait Web3WasmClient {
    fn new(endpoint: &EndPoint) -> Self;
    fn new_custom(endpoint_url_str: &str) -> Self;
    fn new_mainnet() -> Self;
    fn new_devnet() -> Self;
}

impl Web3WasmClient for WasmClient {
    // This throw "Uncaught (in promise) LinkError: WebAssembly.instantiate(): Import #80 module="wbg" function="__wbindgen_closure_wrapper2087" error: function import requires a callable"
    // pub fn new(endpoint: &EndPoint) -> Self {
    //     let endpoint = match endpoint {
    //         EndPoint::Mainnet => EndPoint::Mainnet.to_string(),
    //         EndPoint::Devnet => EndPoint::Devnet.to_string(),
    //         EndPoint::CustomUrl(url) => url.to_string(),
    //     };

    //     Self::new_with_client(WasmClient::new_with_commitment(
    //         endpoint.as_ref(),
    //         CommitmentConfig {
    //             commitment: CommitmentLevel::Confirmed,
    //         },
    //     ))
    // }

    fn new(endpoint: &EndPoint) -> Self {
        WasmClient::new_with_commitment(
            endpoint.to_string().as_ref(),
            CommitmentConfig {
                commitment: CommitmentLevel::Confirmed,
            },
        )
    }

    fn new_custom(endpoint_url_str: &str) -> Self {
        WasmClient::new_with_commitment(
            endpoint_url_str,
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
}
