use anyhow::bail;
use borsh::BorshDeserialize;
use solana_client_wasm::WasmClient;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAccount;
use solana_sdk::{borsh::try_from_slice_unchecked, pubkey::Pubkey};

pub async fn get_and_deserialize_account_data<T: BorshDeserialize>(
    client: &WasmClient,
    pubkey: &Pubkey,
) -> anyhow::Result<T> {
    let data = client.get_account_data(pubkey).await?;
    Ok(try_from_slice_unchecked::<T>(&data)?)
}

pub async fn get_and_deserialize_multiple_accounts_data<T: BorshDeserialize>(
    client: &WasmClient,
    pubkeys: &[Pubkey],
) -> anyhow::Result<Vec<Option<T>>> {
    let maybe_accounts = client.get_multiple_accounts(pubkeys).await?;
    let accounts = maybe_accounts
        .into_iter()
        .map(|account| match account {
            Some(account) => match try_from_slice_unchecked::<T>(&account.data) {
                Ok(a) => Some(a),
                Err(_) => None,
            },
            None => None,
        })
        .collect::<Vec<_>>();
    Ok(accounts)
}

pub async fn get_multiple_token_amount(
    client: &WasmClient,
    pubkeys: &[Pubkey],
) -> anyhow::Result<Vec<UiTokenAccount>> {
    let maybe_accounts = client.get_multiple_token_accounts(pubkeys).await?;

    let accounts = maybe_accounts
        .into_iter()
        .enumerate()
        .flat_map(|(i, account)| {
            Ok(match account {
                Some(account) => account,
                None => bail!("Not found:{}", pubkeys[i]),
            })
        })
        .collect::<Vec<_>>();
    Ok(accounts)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use solana_client_wasm::WasmClient;
    use solana_sdk::pubkey::Pubkey;

    use crate::core::{account::get_multiple_token_amount, client::EndPoint};

    // TODO: find some BorshDeserialize pubkey
    // #[tokio::test]
    // async fn test_get_and_deserialize_account_data() {
    //     let client = WasmClient::new(EndPoint::Mainnet.to_string().as_ref());
    //     let token_metadata_info = get_and_deserialize_account_data::<Mint>(
    //         &client,
    //         &Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
    //     )
    //     .await
    //     .unwrap();

    //     println!("token_metadata_info: {:#?}", token_metadata_info);
    // }

    #[tokio::test]
    async fn test_get_multiple_token_amount() {
        let client = WasmClient::new(EndPoint::Mainnet.to_string().as_ref());
        let results = get_multiple_token_amount(
            &client,
            &[Pubkey::from_str("99WfGSsxb8zRsr3GwsZHSCT6bi1FHVR3RpQhV51cyu6B").unwrap()],
        )
        .await
        .unwrap();

        assert_eq!(
            results[0].owner,
            "3GGFZQ1krmhhviptR4Az1xaF62XFkAKDKDSNShQZVBeE"
        )
    }
}
