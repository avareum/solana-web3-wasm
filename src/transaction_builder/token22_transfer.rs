use anyhow::bail;

use solana_client_wasm::WasmClient;
use solana_sdk::{instruction::Instruction, message::Message, pubkey::Pubkey, system_instruction};

use spl_associated_token_account::{
    get_associated_token_address_with_program_id,
    instruction::create_associated_token_account_idempotent,
};

use async_trait::async_trait;
use solana_client_wasm::utils::rpc_filter::TokenAccountsFilter;
use solana_extra_wasm::program::{spl_token::instruction::transfer_checked, spl_token_2022};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Token22Transfer {
    fn get_message_data_bs58_for_transfer_native(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<String>;

    async fn get_message_data_bs58_for_transfer_spl(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        mint_pubkey: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> anyhow::Result<String>;

    async fn get_instructions_for_transfer_spl(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        mint_pubkey: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> anyhow::Result<Vec<Instruction>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Token22Transfer for WasmClient {
    fn get_message_data_bs58_for_transfer_native(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<String> {
        // 0. Init
        let mut instructions: Vec<Instruction> = vec![];

        // 1. Build transfer ix
        let ix = system_instruction::transfer(source, destination, amount);

        instructions.push(ix);

        // 2. Serialize message to bs58
        let message = Message::new(&instructions, Some(source));
        let message_b58 = bs58::encode(message.serialize()).into_string();

        Ok(message_b58)
    }

    async fn get_instructions_for_transfer_spl(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        mint_pubkey: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> anyhow::Result<Vec<Instruction>> {
        // 0. Init
        let mut instructions: Vec<Instruction> = vec![];
        let spl_token_id = spl_token_2022::id();

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

        Ok(instructions)
    }

    async fn get_message_data_bs58_for_transfer_spl(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        mint_pubkey: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> anyhow::Result<String> {
        // Get instructions.
        let instructions = self
            .get_instructions_for_transfer_spl(source, destination, mint_pubkey, amount, decimals)
            .await?;

        // Serialize message to bs58
        let message = Message::new(&instructions, Some(source));
        let message_b58 = bs58::encode(message.serialize()).into_string();

        Ok(message_b58)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {
    const AIRDROP_AMOUNT: u64 = 1 * LAMPORTS_PER_SOL;

    use crate::{
        core::client::Web3WasmClient, tests::balance::wait_for_balance_change,
        transaction_builder::token22_transfer::Token22Transfer,
    };
    use solana_client_wasm::WasmClient;
    use solana_extra_wasm::program::spl_token_2022::{
        self,
        extension::{transfer_fee, ExtensionType, StateWithExtensionsOwned},
        state::Mint,
    };

    use solana_sdk::{
        native_token::LAMPORTS_PER_SOL, program_pack::Pack, pubkey::Pubkey, rent::Rent,
        signature::Keypair, signer::Signer, system_instruction, transaction::Transaction,
    };
    use spl_associated_token_account::{
        get_associated_token_address_with_program_id, instruction::create_associated_token_account,
    };
    use spl_token_2022::state::Account;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_success_transfer_spl_no_ata() {
        let source_pubkey =
            Pubkey::from_str("DcJGXTE7L1XQtFSdvBv2NPkGCxQ1cziem1yXnqfy2rVy").unwrap();
        let destination_pubkey =
            Pubkey::from_str("DZQVs9FhoWMG19nL3ofmhpQRTjbHgKzM1CitskSGM9mJ").unwrap();
        let mint_pubkey = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();

        // TODO: consider move this to transfer_spl to cross-check input decimals
        // TODO: use devnet with some mint.
        let client = WasmClient::new_devnet();
        let account = client.get_account(&mint_pubkey).await.unwrap();
        let mint_info = Mint::unpack(&account.data).unwrap();
        let decimals = mint_info.decimals;
        let ui_amount = 0.00001f64;
        let amount = spl_token_2022::ui_amount_to_amount(ui_amount, decimals);

        let message_b58 = client
            .get_message_data_bs58_for_transfer_spl(
                &source_pubkey,
                &destination_pubkey,
                &mint_pubkey,
                amount,
                decimals,
            )
            .await
            .unwrap();

        assert!(!message_b58.is_empty());
    }

    // Fork from https://github.com/solana-labs/solana-program-library/blob/24d54db8b8c538c0027198971002c155348e7e3d/associated-token-account/program-test/tests/extended_mint.rs
    #[tokio::test]
    async fn test_associated_token_account_with_transfer_fees() {
        let wallet_sender = Keypair::new();
        let wallet_address_sender = wallet_sender.pubkey();
        let wallet_address_receiver = Pubkey::new_unique();

        // Client
        let client = WasmClient::new_devnet();

        // Payer
        let payer = Keypair::new();
        let balance_before_airdrop_payer = client.get_balance(&payer.pubkey()).await.unwrap();
        println!("balance_before_airdrop_payer:{balance_before_airdrop_payer:?}");

        client
            .request_airdrop(&payer.pubkey(), AIRDROP_AMOUNT)
            .await
            .unwrap();

        // Wait for richer payer
        wait_for_balance_change(
            &client,
            &payer.pubkey(),
            balance_before_airdrop_payer,
            balance_before_airdrop_payer + AIRDROP_AMOUNT,
        )
        .await;

        let recent_blockhash = client.get_latest_blockhash().await.unwrap();
        let rent: Rent = Rent::default();

        // create extended mint
        // ... in the future, a mint can be pre-loaded in program_test.rs like the regular mint
        let mint_account = Keypair::new();
        let token_mint_address = mint_account.pubkey();
        let mint_authority = Keypair::new();
        let space = ExtensionType::get_account_len::<Mint>(&[ExtensionType::TransferFeeConfig]);
        let maximum_fee = 100;
        let mut transaction = Transaction::new_with_payer(
            &[
                system_instruction::create_account(
                    &payer.pubkey(),
                    &mint_account.pubkey(),
                    rent.minimum_balance(space),
                    space as u64,
                    &spl_token_2022::id(),
                ),
                transfer_fee::instruction::initialize_transfer_fee_config(
                    &spl_token_2022::id(),
                    &token_mint_address,
                    Some(&mint_authority.pubkey()),
                    Some(&mint_authority.pubkey()),
                    1_000,
                    maximum_fee,
                )
                .unwrap(),
                spl_token_2022::instruction::initialize_mint(
                    &spl_token_2022::id(),
                    &token_mint_address,
                    &mint_authority.pubkey(),
                    Some(&mint_authority.pubkey()),
                    0,
                )
                .unwrap(),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &mint_account], recent_blockhash);

        // Send #1
        client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap();

        // create extended ATAs
        let mut transaction = Transaction::new_with_payer(
            &[create_associated_token_account(
                &payer.pubkey(),
                &wallet_address_sender,
                &token_mint_address,
                &spl_token_2022::id(),
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        // Send #2
        client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap();

        let recent_blockhash = client.get_latest_blockhash().await.unwrap();

        let mut transaction = Transaction::new_with_payer(
            &[create_associated_token_account(
                &payer.pubkey(),
                &wallet_address_receiver,
                &token_mint_address,
                &spl_token_2022::id(),
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        // Send #3
        client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap();

        let associated_token_address_sender = get_associated_token_address_with_program_id(
            &wallet_address_sender,
            &token_mint_address,
            &spl_token_2022::id(),
        );
        let associated_token_address_receiver = get_associated_token_address_with_program_id(
            &wallet_address_receiver,
            &token_mint_address,
            &spl_token_2022::id(),
        );

        // mint tokens
        let sender_amount = 50 * maximum_fee;
        let mut transaction = Transaction::new_with_payer(
            &[spl_token_2022::instruction::mint_to(
                &spl_token_2022::id(),
                &token_mint_address,
                &associated_token_address_sender,
                &mint_authority.pubkey(),
                &[],
                sender_amount,
            )
            .unwrap()],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &mint_authority], recent_blockhash);

        // Send #4
        client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap();

        // not enough tokens
        let mut transaction = Transaction::new_with_payer(
            &[transfer_fee::instruction::transfer_checked_with_fee(
                &spl_token_2022::id(),
                &associated_token_address_sender,
                &token_mint_address,
                &associated_token_address_receiver,
                &wallet_address_sender,
                &[],
                10_001,
                0,
                maximum_fee,
            )
            .unwrap()],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &wallet_sender], recent_blockhash);

        // Send #5
        let err = client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap_err();

        println!("{err:#?}");

        let recent_blockhash = client.get_latest_blockhash().await.unwrap();

        // success
        let transfer_amount = 500;
        let fee = 50;
        let mut transaction = Transaction::new_with_payer(
            &[transfer_fee::instruction::transfer_checked_with_fee(
                &spl_token_2022::id(),
                &associated_token_address_sender,
                &token_mint_address,
                &associated_token_address_receiver,
                &wallet_address_sender,
                &[],
                transfer_amount,
                0,
                fee,
            )
            .unwrap()],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &wallet_sender], recent_blockhash);

        // Send #6
        client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap();

        let sender_account = client
            .get_account(&associated_token_address_sender)
            .await
            .unwrap();

        let sender_state =
            StateWithExtensionsOwned::<Account>::unpack(sender_account.data).unwrap();
        assert_eq!(sender_state.base.amount, sender_amount - transfer_amount);
        let extension = sender_state
            .get_extension::<transfer_fee::TransferFeeAmount>()
            .unwrap();
        assert_eq!(extension.withheld_amount, 0.into());

        let receiver_account = client
            .get_account(&associated_token_address_receiver)
            .await
            .unwrap();

        let receiver_state =
            StateWithExtensionsOwned::<Account>::unpack(receiver_account.data).unwrap();
        assert_eq!(receiver_state.base.amount, transfer_amount - fee);
        let extension = receiver_state
            .get_extension::<transfer_fee::TransferFeeAmount>()
            .unwrap();
        assert_eq!(extension.withheld_amount, fee.into());
    }
}
