use serde_json::Value;

use solana_client_wasm::WasmClient;
use solana_extra_wasm::program::spl_token::state::Mint;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};

pub async fn get_mint_info(
    client: &WasmClient,
    mint_pubkey: &Pubkey,
) -> Result<Mint, anyhow::Error> {
    let account = client.get_account(mint_pubkey).await?;
    Ok(Mint::unpack(&account.data)?)
}

// TODO: Support token that not exists via Raydium
pub fn get_logo_by_mint_address(mint_address: &str) -> String {
    let token_data = r#"{
    "EsM2FadUJFzVxtzdekL7VThiYSfGrCNcvVqzNLiW4J8a": "https://76ejs4snpuheojjadpqx3uvjvwu6ubbo6zyoni7czu3v3o3tei7q.arweave.net/_4iZck19DkclIBvhfdKpranqBC72cOaj4s03XbtzIj8?ext=jpg"
  }"#;

    let raydium_url = format!("https://img.raydium.io/icon/{mint_address}.png");
    let token_value: Result<Value, serde_json::Error> = serde_json::from_str(token_data);
    match token_value {
        Ok(value) => {
            let url = &value[mint_address];
            if url.is_null() {
                raydium_url
            } else {
                url.as_str().unwrap_or(raydium_url.as_str()).to_owned()
            }
        }

        Err(_) => raydium_url,
    }
}

#[cfg(test)]
mod test {
    use crate::client::Web3WasmClient;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_get_logo_by_mint_address() {
        let mint_address = "EsM2FadUJFzVxtzdekL7VThiYSfGrCNcvVqzNLiW4J8a";
        let logo = get_logo_by_mint_address(mint_address);

        assert_eq!(logo, "https://76ejs4snpuheojjadpqx3uvjvwu6ubbo6zyoni7czu3v3o3tei7q.arweave.net/_4iZck19DkclIBvhfdKpranqBC72cOaj4s03XbtzIj8?ext=jpg")
    }

    #[tokio::test]
    async fn test_get_mint_info() {
        let client = Web3WasmClient::new_mainnet();
        let mint_info = get_mint_info(
            &client,
            &Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(mint_info.decimals, 9)
    }
}
