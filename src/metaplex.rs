use std::collections::HashMap;

use anyhow::bail;
use mpl_token_metadata::state::{Data, Metadata, PREFIX};
use solana_client_wasm::WasmClient;
use solana_sdk::{borsh::try_from_slice_unchecked, pubkey::Pubkey};

pub async fn get_multiple_token_metadata(
    client: &WasmClient,
    mints: &[Pubkey],
) -> Result<Vec<Metadata>, anyhow::Error> {
    let program_key = mpl_token_metadata::id();
    let metadata_keys = mints
        .iter()
        .map(|mint| {
            let metadata_seeds = &[PREFIX.as_bytes(), program_key.as_ref(), mint.as_ref()];
            let (metadata_key, _) = Pubkey::find_program_address(metadata_seeds, &program_key);
            metadata_key
        })
        .collect::<Vec<_>>();

    let metadata_accounts = client.get_multiple_accounts(&metadata_keys).await?;
    let mut errors = vec![];
    let result = metadata_accounts
        .into_iter()
        .flatten()
        .map(|account| try_from_slice_unchecked::<Metadata>(&account.data))
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok());

    let result = result
        .map(|mut metadata| {
            metadata.data = Data {
                name: metadata.data.name.trim_matches(char::from(0)).to_owned(),
                symbol: metadata.data.symbol.trim_matches(char::from(0)).to_owned(),
                uri: metadata.data.uri.trim_matches(char::from(0)).to_owned(),
                seller_fee_basis_points: metadata.data.seller_fee_basis_points,
                creators: metadata.data.creators,
            };
            metadata
        })
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        bail!("errors: {:?}", errors)
    }

    Ok(result)
}

pub async fn get_mint_metadata_map(
    client: &WasmClient,
    mints: &[Pubkey],
) -> Result<HashMap<String, Metadata>, anyhow::Error> {
    let mut mint_metadata_map = HashMap::new();
    get_multiple_token_metadata(client, mints)
        .await?
        .into_iter()
        .for_each(|metadata| {
            mint_metadata_map.insert(metadata.mint.to_string(), metadata);
        });

    Ok(mint_metadata_map)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use solana_client_wasm::WasmClient;
    use solana_sdk::pubkey::Pubkey;

    use crate::{
        metaplex::{get_mint_metadata_map, get_multiple_token_metadata},
        provider::EndPoint,
    };

    #[tokio::test]
    async fn test_get_multiple_token_metadata() {
        let client = WasmClient::new(EndPoint::Mainnet.to_string().as_ref());
        let token_metadata_info = get_multiple_token_metadata(
            &client,
            &[Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap()],
        )
        .await
        .unwrap();

        println!("token_metadata_info: {:#?}", token_metadata_info);
    }

    #[tokio::test]
    async fn test_get_multiple_token_symbol() {
        let client = WasmClient::new(EndPoint::Mainnet.to_string().as_ref());
        let token_metadata_map = get_mint_metadata_map(
            &client,
            &[Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap()],
        )
        .await
        .unwrap();

        println!("token_metadata_map: {:#?}", token_metadata_map);
    }
}
