use anyhow::bail;

use solana_client_wasm::WasmClient;
use solana_sdk::{instruction::Instruction, message::Message, pubkey::Pubkey, system_instruction};

use spl_associated_token_account::{
    get_associated_token_address_with_program_id,
    instruction::create_associated_token_account_idempotent,
};

use async_trait::async_trait;
use solana_client_wasm::utils::rpc_filter::TokenAccountsFilter;
use solana_extra_wasm::program::{spl_token_2022, spl_token_2022::instruction::transfer_checked};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Token22Transfer {
    fn build_transfer_native_instruction(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<Vec<Instruction>>;

    fn build_transfer_native_instruction_message_data_bs58(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<String>;

    async fn build_transfer_spl_instructions(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        mint_pubkey: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> anyhow::Result<Vec<Instruction>>;

    async fn build_transfer_spl_instructions_message_data_bs58(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        mint_pubkey: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> anyhow::Result<String>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Token22Transfer for WasmClient {
    fn build_transfer_native_instruction(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<Vec<Instruction>> {
        // 0. Init
        let mut instructions: Vec<Instruction> = vec![];

        // 1. Build transfer ix
        let ix = system_instruction::transfer(source, destination, amount);

        instructions.push(ix);

        Ok(instructions)
    }

    fn build_transfer_native_instruction_message_data_bs58(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<String> {
        // 1. Build transfer ix
        match self.build_transfer_native_instruction(source, destination, amount) {
            Ok(instructions) => {
                // 2. Serialize message to bs58
                let message = Message::new(&instructions, Some(source));
                let message_b58 = bs58::encode(message.serialize()).into_string();

                Ok(message_b58)
            }
            Err(e) => bail!(e),
        }
    }

    async fn build_transfer_spl_instructions(
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

    async fn build_transfer_spl_instructions_message_data_bs58(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        mint_pubkey: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> anyhow::Result<String> {
        // Get instructions.
        let instructions = self
            .build_transfer_spl_instructions(source, destination, mint_pubkey, amount, decimals)
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

    use super::*;
    use crate::{core::client::Web3WasmClient, tests::balance::wait_for_balance_change};
    use solana_client_wasm::WasmClient;
    use solana_extra_wasm::program::{
        spl_memo,
        spl_token_2022::{
            self,
            extension::{
                memo_transfer::instruction::enable_required_transfer_memos,
                transfer_fee::{
                    self,
                    instruction::{
                        withdraw_withheld_tokens_from_accounts, withdraw_withheld_tokens_from_mint,
                    },
                },
                ExtensionType, StateWithExtensionsOwned,
            },
            state::Mint,
        },
    };

    use solana_sdk::{
        hash::Hash,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        rent::Rent,
        signature::Keypair,
        signer::Signer,
        system_instruction,
        transaction::Transaction,
    };
    use spl_associated_token_account::{
        get_associated_token_address_with_program_id, instruction::create_associated_token_account,
    };
    use spl_token_2022::state::Account;

    struct TestContext {
        client: WasmClient,
        payer: Keypair,
        recent_blockhash: Hash,
        rent: Rent,
    }

    impl TestContext {
        pub async fn new() -> Self {
            let client = WasmClient::new_devnet();
            let payer = Keypair::from_bytes(
                [
                    198, 153, 231, 18, 212, 198, 237, 103, 115, 63, 253, 27, 78, 112, 53, 11, 67,
                    208, 171, 188, 17, 137, 93, 44, 42, 47, 30, 194, 42, 216, 249, 152, 6, 184, 75,
                    232, 188, 125, 225, 196, 192, 112, 221, 23, 104, 136, 67, 248, 190, 29, 4, 54,
                    121, 172, 103, 15, 119, 125, 9, 15, 243, 107, 6, 91,
                ]
                .as_ref(),
            )
            .unwrap();

            let balance_before_airdrop_payer = client.get_balance(&payer.pubkey()).await.unwrap();
            println!("balance_before_airdrop_payer:{balance_before_airdrop_payer:?}");

            match client
                .request_airdrop(&payer.pubkey(), AIRDROP_AMOUNT)
                .await
            {
                Ok(_) => {
                    // Wait for airdrop
                    wait_for_balance_change(&client, &payer.pubkey(), balance_before_airdrop_payer)
                        .await;
                }
                Err(error) => println!("{error}"),
            };

            let recent_blockhash = client.get_latest_blockhash().await.unwrap();
            let rent = Rent::default();

            TestContext {
                client,
                payer,
                recent_blockhash,
                rent,
            }
        }
    }

    // Fork from https://github.com/solana-labs/solana-program-library/blob/24d54db8b8c538c0027198971002c155348e7e3d/associated-token-account/program-test/tests/extended_mint.rs
    #[tokio::test]
    async fn test_associated_token_account_with_transfer_fees() {
        // Context
        let test_context = TestContext::new().await;
        let client = test_context.client;
        let payer = test_context.payer;
        let recent_blockhash = test_context.recent_blockhash;
        let rent = test_context.rent;

        let fee_vault = Keypair::new();

        let wallet_sender = Keypair::new();
        let wallet_address_sender = wallet_sender.pubkey();

        let wallet_receiver = Keypair::new();
        let wallet_address_receiver = wallet_receiver.pubkey();

        // 1. create extended mint
        // ... in the future, a mint can be pre-loaded in program_test.rs like the regular mint
        let mint_account = Keypair::new();
        let token_mint_address = mint_account.pubkey();
        let mint_authority = Keypair::new();
        let space = ExtensionType::get_account_len::<Mint>(&[ExtensionType::TransferFeeConfig]);
        let maximum_fee = 100;
        let decimals = 0;
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
                    &token_mint_address,       // mint
                    Some(&fee_vault.pubkey()), // transfer_fee_config_authority
                    Some(&fee_vault.pubkey()), // withdraw_withheld_authority
                    1_000,
                    maximum_fee,
                )
                .unwrap(),
                spl_token_2022::instruction::initialize_mint(
                    &spl_token_2022::id(),
                    &token_mint_address,
                    &mint_authority.pubkey(),
                    Some(&mint_authority.pubkey()),
                    decimals,
                )
                .unwrap(),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &mint_account], recent_blockhash);

        // Send 1.
        client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap();

        // 2. create extended ATAs sender
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

        // Send 2.
        client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap();

        // 3. create extended ATAs receiver
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

        // Send 3.
        client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap();

        // 3.1 create extended ATAs platform
        let recent_blockhash = client.get_latest_blockhash().await.unwrap();

        let mut transaction = Transaction::new_with_payer(
            &[create_associated_token_account(
                &payer.pubkey(),
                &fee_vault.pubkey(),
                &token_mint_address,
                &spl_token_2022::id(),
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        // Send 3.1
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
        let associated_token_address_fee_vault = get_associated_token_address_with_program_id(
            &fee_vault.pubkey(),
            &token_mint_address,
            &spl_token_2022::id(),
        );

        // 4. mint tokens
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

        // Send 4.
        client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap();

        // // 5. not enough tokens
        // let mut transaction = Transaction::new_with_payer(
        //     &[transfer_fee::instruction::transfer_checked_with_fee(
        //         &spl_token_2022::id(),
        //         &associated_token_address_sender,
        //         &token_mint_address,
        //         &associated_token_address_receiver,
        //         &wallet_address_sender,
        //         &[],
        //         10_001,
        //         0,
        //         maximum_fee,
        //     )
        //     .unwrap()],
        //     Some(&payer.pubkey()),
        // );
        // transaction.sign(&[&payer, &wallet_sender], recent_blockhash);

        // // Send 5.
        // let err = client
        //     .send_and_confirm_transaction(&transaction)
        //     .await
        //     .unwrap_err();

        // println!("Expected error: {err:#?}");

        // 6. Transfer with memo.
        let recent_blockhash = client.get_latest_blockhash().await.unwrap();

        let transfer_amount = 500;
        let fee = 50;
        let memo_ix = spl_memo::build_memo("Hello World!".as_bytes(), &[&payer.pubkey()]);
        let mut transaction = Transaction::new_with_payer(
            &[
                transfer_fee::instruction::transfer_checked_with_fee(
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
                .unwrap(),
                memo_ix,
            ],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[&payer, &wallet_sender], recent_blockhash);

        // Send 6.
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

        println!(
            "extension.withheld_amount: {:#?}",
            extension.withheld_amount
        );

        // forked from https://github.com/solana-labs/solana/blob/6bd4ae695528e394258d90fb7beaece488475674/transaction-status/src/parse_token/extension/transfer_fee.rs
        // Single authority WithdrawWithheldTokensFromAccounts
        let withdraw_withheld_tokens_from_accounts_ix = withdraw_withheld_tokens_from_accounts(
            &spl_token_2022::id(),
            &mint_account.pubkey(),
            &associated_token_address_fee_vault,
            &fee_vault.pubkey(),
            &[],
            &[&associated_token_address_receiver],
        )
        .unwrap();

        let mut transaction = Transaction::new_with_payer(
            &[withdraw_withheld_tokens_from_accounts_ix],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[&payer, &fee_vault], recent_blockhash);

        // Send 7.
        client
            .send_and_confirm_transaction(&transaction)
            .await
            .unwrap();

        ////// DIDN'T WORK
        // // require memo transfers into wallet_address_receiver
        // let enable_memo_transfers_ix = enable_required_transfer_memos(
        //     &spl_token_2022::id(),
        //     &associated_token_address_receiver,
        //     &wallet_address_receiver,
        //     &[&wallet_address_receiver],
        // )
        // .unwrap();

        // let wallet_sender_state = client.get_account(&wallet_sender.pubkey()).await.unwrap();
        // let extension = wallet_sender_state.get_extension::<MemoTransfer>().unwrap();
        // assert!(bool::from(extension.require_incoming_transfer_memos));
        // println!("{wallet_sender_state:#?}");

        // let recent_blockhash = client.get_latest_blockhash().await.unwrap();

        // let mut transaction =
        //     Transaction::new_with_payer(&[enable_memo_transfers_ix], Some(&payer.pubkey()));
        // transaction.sign(&[&payer, &wallet_receiver], recent_blockhash);

        // // Send #7
        // client
        //     .send_and_confirm_transaction(&transaction)
        //     .await
        //     .unwrap();

        // https://github.com/solana-labs/solana-program-library/blob/5f4943802bfb8ac7eaf9ede62f68ef9a94d1427d/token/program-2022-test/tests/memo_transfer.rs
        // https://github.com/solana-labs/solana/blob/6bd4ae695528e394258d90fb7beaece488475674/transaction-status/src/parse_token/extension/memo_transfer.rs
        // transfer with memo
        // let memo = "Hello World!";
        // let signer_pubkeys = [payer.pubkey()];
        // let memo_ix = Instruction {
        //     program_id: spl_memo::id(),
        //     accounts: signer_pubkeys
        //         .iter()
        //         .map(|pubkey| AccountMeta::new_readonly(*pubkey, true))
        //         .collect(),
        //     data: memo.as_bytes().to_vec(),
        // };
        // let memo_ix = spl_memo::build_memo(memo.as_bytes(), &[&payer.pubkey()]);

        // // transfer
        // let amount = 100u64;
        // let transfer_ixs = client
        //     .get_instructions_for_transfer_spl(
        //         &wallet_address_sender,
        //         &wallet_address_receiver,
        //         &token_mint_address,
        //         amount,
        //         decimals,
        //     )
        //     .await
        //     .unwrap();

        // let recent_blockhash = client.get_latest_blockhash().await.unwrap();

        // let mut ixs = vec![memo_ix];
        // ixs.extend(transfer_ixs);
        // let mut transaction = Transaction::new_with_payer(&ixs, Some(&payer.pubkey()));
        // transaction.sign(&[&payer], recent_blockhash);

        // // Send #8
        // client
        //     .send_and_confirm_transaction(&transaction)
        //     .await
        //     .unwrap();

        println!("wallet_address_sender: {wallet_address_sender:#?}");
        println!("wallet_address_receiver: {wallet_address_receiver:#?}");
        println!("token_mint_address: {token_mint_address:#?}");
        println!("fee_vault: {:#?}", fee_vault.pubkey());
        println!("associated_token_address_fee_vault: {associated_token_address_fee_vault:#?}");
    }
}
