use anyhow::bail;

use solana_client_wasm::WasmClient;
use solana_sdk::{instruction::Instruction, message::Message, pubkey::Pubkey, system_instruction};

use spl_associated_token_account::{
    get_associated_token_address_with_program_id,
    instruction::create_associated_token_account_idempotent,
};

use async_trait::async_trait;
use solana_client_wasm::utils::rpc_filter::TokenAccountsFilter;
use solana_extra_wasm::program::spl_token::instruction::transfer_checked;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait TokenTransfer {
    fn transfer_native(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
    ) -> Result<String, anyhow::Error>;

    async fn transfer_spl(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        mint_pubkey: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> Result<String, anyhow::Error>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TokenTransfer for WasmClient {
    fn transfer_native(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
    ) -> Result<String, anyhow::Error> {
        // 0. Init
        let mut instructions: Vec<Instruction> = vec![];

        // 1. Build transfer ix
        let ix = system_instruction::transfer(source, destination, amount);

        instructions.push(ix);

        // 2. Serialize message to base64
        let message = Message::new(&instructions, Some(source));
        let message_b64 = base64::encode(message.serialize());

        Ok(message_b64)
    }

    async fn transfer_spl(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        mint_pubkey: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> Result<String, anyhow::Error> {
        // 0. Init
        let mut instructions: Vec<Instruction> = vec![];
        let spl_token_id = spl_token::id();

        // 1. Get source ATA
        let source_token_account_pubkey =
            get_associated_token_address_with_program_id(source, mint_pubkey, &spl_token_id);

        // 1.1 Get target ATA
        let ata_pubkey =
            get_associated_token_address_with_program_id(destination, mint_pubkey, &spl_token_id);
        let token_account_filter = TokenAccountsFilter::Mint(*mint_pubkey);

        let accounts = match self
            .get_token_accounts_by_owner(destination, token_account_filter)
            .await
        {
            Ok(accounts) => accounts,
            Err(_) => vec![],
        };

        let is_ata_exists = accounts.iter().any(|e| e.pubkey == ata_pubkey.to_string());

        if !is_ata_exists {
            // 1.2 Create ATA if not exists
            let create_associated_token_account_idempotent_ix =
                create_associated_token_account_idempotent(
                    source,
                    destination,
                    mint_pubkey,
                    &spl_token_id,
                );

            // 1.3 Add crate ATA ix
            instructions.push(create_associated_token_account_idempotent_ix);
        }

        // 2. Transfer SPL
        let ix = match transfer_checked(
            &spl_token_id,
            &source_token_account_pubkey,
            mint_pubkey,
            &ata_pubkey,
            source,
            &[],
            amount,
            decimals,
        ) {
            Err(err) => bail!(err),
            Ok(data) => data,
        };

        instructions.push(ix);

        // 3. Serialize message to base64
        let message = Message::new(&instructions, Some(source));
        let message_b64 = base64::encode(message.serialize());

        Ok(message_b64)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {
    use super::TokenTransfer;

    use crate::core::client::Web3WasmClient;
    use solana_client_wasm::WasmClient;
    use solana_extra_wasm::program::spl_token::state::Mint;

    use solana_sdk::{program_pack::Pack, pubkey::Pubkey};
    use std::str::FromStr;

    #[tokio::test]
    async fn success_transfer_spl_no_ata() {
        let source_pubkey =
            Pubkey::from_str("DcJGXTE7L1XQtFSdvBv2NPkGCxQ1cziem1yXnqfy2rVy").unwrap();
        let destination_pubkey =
            Pubkey::from_str("DZQVs9FhoWMG19nL3ofmhpQRTjbHgKzM1CitskSGM9mJ").unwrap();
        let mint_pubkey = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();

        // TODO: consider move this to transfer_spl to cross-check input decimals
        let client = WasmClient::new_mainnet();
        let account = client.get_account(&mint_pubkey).await.unwrap();
        let mint_info = Mint::unpack(&account.data).unwrap();
        let decimals = mint_info.decimals;
        let ui_amount = 0.001f64;
        let amount = spl_token::ui_amount_to_amount(ui_amount, decimals);

        let message_b64 = client
            .transfer_spl(
                &source_pubkey,
                &destination_pubkey,
                &mint_pubkey,
                amount,
                decimals,
            )
            .await
            .unwrap();

        assert!(!message_b64.is_empty());
    }
}
