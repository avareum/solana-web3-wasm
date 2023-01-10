use anyhow::{self, bail};
use mpl_token_metadata::state::Metadata;
use solana_extra_wasm::program::spl_associated_token_account::get_associated_token_address;
use std::collections::HashMap;

use crate::{client::Web3WasmClient, metaplex::get_mint_metadata_map};
use solana_client_wasm::WasmClient;
use solana_sdk::pubkey::Pubkey;

pub struct NftInformation {
    client: WasmClient,
}

impl NftInformation {
    pub fn new_mainnet() -> Self {
        Self {
            client: WasmClient::new_mainnet(),
        }
    }

    pub fn new_devnet() -> Self {
        Self {
            client: WasmClient::new_devnet(),
        }
    }

    // read
    async fn find_nfts_by_mint_list(
        self: NftInformation,
        owner_address: &Pubkey,
        mint_list: &[Pubkey],
    ) -> anyhow::Result<HashMap<String, Metadata>> {
        // 1. AST exist?
        let ast_list = mint_list
            .iter()
            .map(|mint| get_associated_token_address(owner_address, mint))
            .collect::<Vec<_>>();

        // 2. Has NFT in that ast?
        let mut token_balances = self
            .client
            .get_multiple_token_accounts(&ast_list)
            .await?
            .into_iter()
            .flatten()
            .filter(|e| e.token_amount.amount == "1");

        if token_balances.next().is_none() {
            bail!("No token found.")
        }

        // 3. Get nft info
        let token_metadata_map = get_mint_metadata_map(&self.client, mint_list)
            .await
            .unwrap();

        println!("token_metadata_map: {:#?}", token_metadata_map);

        Ok(token_metadata_map)
    }
}

#[cfg(test)]
mod test {
    use crate::info::nft::NftInformation;
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_find_nfts_by_mint_list() {
        let mint_address = "A2NzysADP3a6FzgKkh4dzQbwK6CgsJcdo3Rz6opfFMPy";
        let nft_info = NftInformation::new_devnet();
        let token_metadata_info = nft_info
            .find_nfts_by_mint_list(
                // owner_address
                &Pubkey::from_str("9K9RDUPvRfcVmHnoThUGkdR2bfQwa9oH1bs8RsmR2fjc").unwrap(),
                // mint_list
                &[Pubkey::from_str(mint_address).unwrap()],
            )
            .await
            .unwrap();

        assert_eq!(
            token_metadata_info
                .get(mint_address)
                .unwrap()
                .mint
                .to_string(),
            mint_address.to_owned()
        );
        assert_eq!(token_metadata_info.len(), 1);
    }
}
