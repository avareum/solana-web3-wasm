use serde::{Deserialize, Serialize, Serializer};
use solana_extra_wasm::account_decoder::parse_token::UiTokenAccount;

use super::sort::UnsupportedAccount;

#[cfg(feature = "wasm_bindgen")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletInfo {
    pub decimals: u8,              // 9
    pub lamports: u64,             // 1895991
    pub lamports_string: String,   // "1895991"
    pub ui_balance: f64,           // 1.895991
    pub ui_balance_string: String, // "1.895991"
    pub symbol: String,            // "SOL"
    pub logo: String, // "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png"
    pub name: String, // "Solana Native Token"
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletTokenInfo {
    pub decimals: u8,             // 6
    pub amount_string: String,    // "1895991"
    pub ui_amount: f64,           // 1.895991
    pub ui_amount_string: String, // "1.895991"
    pub symbol: String,           // "USDC"
    pub logo: String, // "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v/logo.png"
    pub address: String, // "B8MA5aWJ7xv3SQgmnLe5orh7zDt8ah6JybBsTPhkT1Ng"
    pub mint: String, // "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
    pub name: String, // "USD Coin"
}

#[cfg(feature = "wasm_bindgen")]
impl WalletInfo {
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        match serde_wasm_bindgen::to_value(self) {
            Ok(js_value) => Ok(js_value),
            Err(err) => Err(JsError::new(&err.to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletTokenAccount {
    pub address: String,
    pub is_associated: bool,
    #[serde(flatten)]
    pub account: UiTokenAccount,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WalletTokenAccounts {
    #[serde(serialize_with = "flattened")]
    pub(crate) accounts: Vec<Vec<WalletTokenAccount>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) unsupported_accounts: Vec<UnsupportedAccount>,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub(crate) max_len_balance: usize,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub(crate) aux_len: usize,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub(crate) token_is_some: bool,
}

fn flattened<S: Serializer>(
    vec: &[Vec<WalletTokenAccount>],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let flattened: Vec<_> = vec.iter().flatten().collect();
    flattened.serialize(serializer)
}
