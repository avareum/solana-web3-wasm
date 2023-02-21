use crate::{
    core::{metaplex::get_mint_metadata_map, mint::get_logo_by_mint_address},
    solana_client_wasm::{utils::rpc_filter::TokenAccountsFilter, WasmClient},
    wallet::sort::sort_and_parse_token_accounts,
};
use anyhow::bail;
use async_trait::async_trait;
use solana_extra_wasm::program::spl_token;
use solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey};
use std::str::FromStr;

use crate::wallet::structs::*;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait WalletInformation {
    async fn get_wallet_info(&self, wallet_address: &str) -> Result<WalletInfo, anyhow::Error>;
    async fn get_wallet_token_infos(
        &self,
        wallet_address: &str,
        token_address: Option<String>,
    ) -> Result<Vec<Option<WalletTokenInfo>>, anyhow::Error>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WalletInformation for WasmClient {
    async fn get_wallet_info(&self, wallet_address: &str) -> Result<WalletInfo, anyhow::Error> {
        let account = self.get_account(&Pubkey::from_str(wallet_address)?).await;

        let decimals = 9u8;
        let symbol = "SOL".to_owned();
        let logo = get_logo_by_mint_address("So11111111111111111111111111111111111111112");
        let name = "Solana Native Token".to_owned();

        match account {
            Ok(account) => {
                let lamports_string = account.lamports.to_string();
                let ui_balance = lamports_to_sol(lamports_string.parse::<u64>()?);

                Ok(WalletInfo {
                    decimals,
                    lamports: account.lamports,
                    lamports_string,
                    ui_balance,
                    ui_balance_string: ui_balance.to_string(),
                    symbol,
                    logo,
                    name,
                })
            }
            Err(err) => {
                if err.to_string().starts_with("Client error:") {
                    Ok(WalletInfo {
                        decimals,
                        lamports: 0u64,
                        lamports_string: "0".to_string(),
                        ui_balance: 0f64,
                        ui_balance_string: "0".to_string(),
                        symbol,
                        logo,
                        name,
                    })
                } else {
                    bail!(err)
                }
            }
        }
    }

    async fn get_wallet_token_infos(
        &self,
        wallet_address: &str,
        token_address: Option<String>,
    ) -> Result<Vec<Option<WalletTokenInfo>>, anyhow::Error> {
        let owner = Pubkey::from_str(wallet_address)?;
        let token = match token_address {
            Some(token_address) => Some(Pubkey::from_str(&token_address)?),
            None => None,
        };

        let token_account_filter = match token {
            Some(token) => TokenAccountsFilter::Mint(token),
            None => TokenAccountsFilter::ProgramId(spl_token::id()),
        };
        let accounts = self
            .get_token_accounts_by_owner(&owner, token_account_filter)
            .await?;

        if accounts.is_empty() {
            return Ok(vec![]);
        }

        let (mint_accounts, unsupported_accounts, max_len_balance, includes_aux) =
            sort_and_parse_token_accounts(&owner, accounts, &spl_token::id());
        let aux_len = if includes_aux { 10 } else { 0 };

        let wallet_token_accounts = WalletTokenAccounts {
            accounts: mint_accounts.into_values().collect(),
            unsupported_accounts,
            max_len_balance,
            aux_len,
            token_is_some: token.is_some(),
        };

        let flatten_accounts = wallet_token_accounts
            .accounts
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        // Find mint symbol map
        let mints = flatten_accounts
            .iter()
            .map(|flatten_account| Pubkey::from_str(&flatten_account.account.mint).unwrap())
            .collect::<Vec<_>>();

        let mint_metadata_map = get_mint_metadata_map(self, &mints).await?;

        let wallet_token_info_list = flatten_accounts
            .into_iter()
            .map(|flatten_account| -> Option<WalletTokenInfo> {
                let ui_token_account = &flatten_account.account;
                let token_amount = &flatten_account.account.token_amount;
                let address = flatten_account.address.to_owned();
                let mint = ui_token_account.mint.to_string();

                // Handle unsupported mint
                mint_metadata_map.get(&mint).map(|mint_metadata| {
                    let symbol = mint_metadata.data.symbol.to_owned();
                    let name = mint_metadata.data.name.to_owned();
                    let logo = get_logo_by_mint_address(&mint);

                    WalletTokenInfo {
                        mint,
                        name,
                        symbol,
                        logo,
                        address,
                        decimals: token_amount.decimals.to_owned(),
                        amount_string: token_amount.amount.to_owned(),
                        ui_amount: token_amount.ui_amount.unwrap_or(0.0).to_owned(),
                        ui_amount_string: token_amount.ui_amount_string.to_owned(),
                    }
                })
            })
            .collect::<Vec<_>>();

        Ok(wallet_token_info_list)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {
    use {crate::core::client::Web3WasmClient, solana_client_wasm::WasmClient};

    use super::WalletInformation;

    #[tokio::test]
    async fn test_get_wallet_info() {
        let pubkey = "DcJGXTE7L1XQtFSdvBv2NPkGCxQ1cziem1yXnqfy2rVy";
        let wallet_info = WasmClient::new_mainnet()
            .get_wallet_info(pubkey)
            .await
            .unwrap();

        println!("wallet_info: {wallet_info:#?}");
        assert!(wallet_info.lamports > 0);
    }

    #[tokio::test]
    async fn test_get_wallet_info_not_exist() {
        let pubkey = "3mPuPCgmdexxcSYtpKDTPktTnEYHJcsZxCJdfemN1xgt";
        let wallet_info = WasmClient::new_mainnet()
            .get_wallet_info(pubkey)
            .await
            .unwrap();

        println!("wallet_info: {wallet_info:#?}");
        assert!(wallet_info.lamports == 0);
    }

    #[tokio::test]
    async fn test_get_wallet_token_infos() {
        let pubkey = "DcJGXTE7L1XQtFSdvBv2NPkGCxQ1cziem1yXnqfy2rVy";
        let token_info = WasmClient::new_mainnet()
            .get_wallet_token_infos(pubkey, None)
            .await
            .unwrap();

        println!("token_info: {token_info:#?}");
        assert!(!token_info.is_empty());
    }
}
