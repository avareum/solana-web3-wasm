use serde::{Deserialize, Serialize, Serializer};
use solana_extra_wasm::account_decoder::parse_token::UiTokenAccount;

use super::sort::UnsupportedAccount;

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
