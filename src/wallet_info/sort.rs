use std::{
    collections::{btree_map::Entry, BTreeMap},
    str::FromStr,
};

use anyhow::bail;
use serde::{Deserialize, Serialize};
use solana_client_wasm::utils::rpc_config::RpcKeyedAccount;
use solana_extra_wasm::{
    account_decoder::{
        parse_token::{TokenAccountType, UiTokenAmount},
        UiAccountData,
    },
    program::spl_associated_token_account,
};
use solana_sdk::pubkey::Pubkey;

use super::structs::WalletTokenAccount;

pub(crate) type MintAccounts = BTreeMap<String, Vec<WalletTokenAccount>>;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UnsupportedAccount {
    pub address: String,
    pub err: String,
}

pub(crate) fn sort_and_parse_token_accounts(
    owner: &Pubkey,
    accounts: Vec<RpcKeyedAccount>,
    program_id: &Pubkey,
) -> (MintAccounts, Vec<UnsupportedAccount>, usize, bool) {
    let mut mint_accounts: MintAccounts = BTreeMap::new();
    let mut unsupported_accounts = vec![];
    let mut max_len_balance = 0;
    let mut includes_aux = false;
    for keyed_account in accounts {
        let address = keyed_account.pubkey;

        if let UiAccountData::Json(parsed_account) = keyed_account.account.data {
            if parsed_account.program != "spl-token" {
                unsupported_accounts.push(UnsupportedAccount {
                    address,
                    err: format!("Unsupported account program: {}", parsed_account.program),
                });
            } else {
                match serde_json::from_value(parsed_account.parsed) {
                    Ok(TokenAccountType::Account(ui_token_account)) => {
                        let mint = ui_token_account.mint.clone();
                        let is_associated = if let Ok(mint) = Pubkey::from_str(&mint) {
                            spl_associated_token_account::get_associated_token_address_with_program_id(owner, &mint, program_id).to_string() == address
                        } else {
                            includes_aux = true;
                            false
                        };
                        let len_balance = ui_token_account
                            .token_amount
                            .real_number_string_trimmed()
                            .len();
                        max_len_balance = max_len_balance.max(len_balance);
                        let parsed_account = WalletTokenAccount {
                            address,
                            account: ui_token_account,
                            is_associated,
                        };
                        let entry = mint_accounts.entry(mint);
                        match entry {
                            Entry::Occupied(_) => {
                                entry.and_modify(|e| e.push(parsed_account));
                            }
                            Entry::Vacant(_) => {
                                entry.or_insert_with(|| vec![parsed_account]);
                            }
                        }
                    }
                    Ok(_) => unsupported_accounts.push(UnsupportedAccount {
                        address,
                        err: "Not a token account".to_string(),
                    }),
                    Err(err) => unsupported_accounts.push(UnsupportedAccount {
                        address,
                        err: format!("Account parse failure: {}", err),
                    }),
                }
            }
        } else {
            unsupported_accounts.push(UnsupportedAccount {
                address,
                err: "Unsupported account data format".to_string(),
            });
        }
    }
    for (_, array) in mint_accounts.iter_mut() {
        array.sort_by(|a, b| b.is_associated.cmp(&a.is_associated));
    }
    (
        mint_accounts,
        unsupported_accounts,
        max_len_balance,
        includes_aux,
    )
}

#[allow(dead_code)]
pub(crate) fn parse_token_account(
    keyed_account: RpcKeyedAccount,
) -> Result<UiTokenAmount, anyhow::Error> {
    if let UiAccountData::Json(parsed_account) = keyed_account.account.data {
        if parsed_account.program != "spl-token" {
            bail!("Unsupported account program: {}", parsed_account.program)
        } else {
            match serde_json::from_value(parsed_account.parsed) {
                Ok(TokenAccountType::Account(ui_token_account)) => {
                    Ok(ui_token_account.token_amount)
                }
                Ok(_) => bail!("Not a token account".to_string()),
                Err(err) => bail!("Account parse failure: {}", err),
            }
        }
    } else {
        bail!("Unsupported account data format".to_string());
    }
}
