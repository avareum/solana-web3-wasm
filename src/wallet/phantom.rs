use solana_sdk::transaction::{Transaction, VersionedTransaction};

#[cfg(feature = "wasm_bindgen")]
use wasm_bindgen::prelude::*;

use crate::wallet::transaction_v0::TransactionV0Value;

use super::transaction::TransactionValue;

// Fun -------------------------------------

#[wasm_bindgen]
pub enum EncodingType {
    Base58,
    Base64,
}

pub fn get_message_data_bs58_from_transaction(tx: &str) -> anyhow::Result<String> {
    get_message_data_from_transaction(tx, &EncodingType::Base58)
}

pub fn get_message_data_bs58_from_transactions(txs: Vec<String>) -> anyhow::Result<Vec<String>> {
    get_message_data_from_transactions(txs, &EncodingType::Base58)
}

pub fn get_message_data_from_transaction(
    tx: &str,
    encoding_type: &EncodingType,
) -> anyhow::Result<String> {
    let tx_json = serde_json::from_str(tx)?;
    let tx_value = serde_json::from_value::<TransactionValue>(tx_json);

    let message_data = match tx_value {
        // Legacy
        Ok(tx_value) => {
            let tx = Transaction::try_from(tx_value)?;
            tx.message_data()
        }
        // V0
        Err(_) => {
            let tx_json: TransactionV0Value = serde_json::from_str(tx)?;
            let tx = VersionedTransaction::try_from(tx_json)?;
            tx.message.serialize()
        }
    };

    // Encode
    let message_data_string = match encoding_type {
        EncodingType::Base58 => bs58::encode(message_data).into_string(),
        EncodingType::Base64 => base64::encode(message_data),
    };

    Ok(message_data_string)
}

pub fn get_message_data_from_transactions(
    txs: Vec<String>,
    encoding_type: &EncodingType,
) -> anyhow::Result<Vec<String>> {
    let mut errors = vec![];
    let result = txs
        .into_iter()
        .map(|e| get_message_data_from_transaction(&e, encoding_type))
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect::<Vec<_>>();

    Ok(result)
}

// Test -------------------------------------

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    use crate::tests::mock::*;
    use solana_sdk::{
        instruction::CompiledInstruction,
        message::{
            v0::{self, MessageAddressTableLookup},
            MessageHeader, VersionedMessage,
        },
        pubkey::Pubkey,
        system_instruction,
        transaction::Transaction,
    };

    #[tokio::test]
    async fn success_get_message_data_bs58_from_transaction() {
        // Setup
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let tx = get_transfer_transaction_string(Some(recent_blockhash));
        let message_data_bs58 = get_message_data_bs58_from_transaction(tx.as_str()).unwrap();

        // Prove
        let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        let mut tx = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        tx.message.recent_blockhash = recent_blockhash;

        let message_data = tx.message_data();
        let sdk_message_data_bs58 = bs58::encode(message_data).into_string();

        assert_eq!(message_data_bs58, sdk_message_data_bs58);
    }

    #[tokio::test]
    async fn success_get_message_data_bs58_from_transactions() {
        // Setup
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let tx1_string = get_transfer_transaction_string(Some(recent_blockhash));
        let tx2_string = get_transfer_transaction_string(Some(recent_blockhash));
        let txs = vec![tx1_string, tx2_string];

        let message_data_bs58s = get_message_data_bs58_from_transactions(txs).unwrap();

        // Prove
        let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        let mut tx = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        tx.message.recent_blockhash = recent_blockhash;

        let message_datas = vec![tx.message_data(), tx.message_data()];
        let sdk_message_data_bs58s = message_datas
            .iter()
            .map(|e| bs58::encode(e).into_string())
            .collect::<Vec<_>>();

        assert_eq!(message_data_bs58s, sdk_message_data_bs58s);
    }

    #[tokio::test]
    async fn success_get_message_data_bs58_from_transaction_v0() {
        // Setup
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let mocked_tx_v0 = get_transfer_transaction_v0_string(Some(recent_blockhash));
        let message_data_bs58 =
            get_message_data_bs58_from_transaction(mocked_tx_v0.as_str()).unwrap();

        // Prove
        let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        let mut tx = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        tx.message.recent_blockhash = recent_blockhash;

        // Create v0 compatible message
        let alice_keypair = get_alice_keypair();
        let versioned_transaction = match VersionedTransaction::try_new(
            VersionedMessage::Legacy(tx.message),
            &[&alice_keypair],
        ) {
            Ok(tx) => {
                assert_eq!(tx.verify_with_results(), vec![true; 1]);
                tx
            }
            Err(err) => {
                assert_eq!(Some(err), None);
                panic!("error");
            }
        };

        let ix0 = versioned_transaction.message.instructions().get(0).unwrap();
        let version_0_message = VersionedMessage::V0(v0::Message {
            header: MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 1,
            },
            recent_blockhash,
            account_keys: vec![alice_pubkey, Pubkey::default()],
            address_table_lookups: vec![],
            instructions: vec![CompiledInstruction {
                program_id_index: ix0.program_id_index,
                accounts: ix0.accounts.clone(),
                data: ix0.data.clone(),
            }],
        });

        let sdk_message_data_bs58 = bs58::encode(version_0_message.serialize()).into_string();

        assert_eq!(message_data_bs58, sdk_message_data_bs58);
    }

    #[tokio::test]
    async fn success_get_message_data_bs58_from_transaction_v0_with_address_table_lookups() {
        // Setup
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let mocked_tx_v0 =
            get_transfer_transaction_v0_with_address_table_lookups_string(Some(recent_blockhash));
        let message_data_bs58 =
            get_message_data_bs58_from_transaction(mocked_tx_v0.as_str()).unwrap();

        // Prove
        let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        let mut tx = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        tx.message.recent_blockhash = recent_blockhash;

        // Create v0 compatible message
        let alice_keypair = get_alice_keypair();
        let versioned_transaction = match VersionedTransaction::try_new(
            VersionedMessage::Legacy(tx.message),
            &[&alice_keypair],
        ) {
            Ok(tx) => {
                assert_eq!(tx.verify_with_results(), vec![true; 1]);
                tx
            }
            Err(err) => {
                assert_eq!(Some(err), None);
                panic!("error");
            }
        };

        let version_0_message = VersionedMessage::V0(v0::Message {
            header: MessageHeader {
                num_required_signatures: 2,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 4,
            },
            recent_blockhash,
            account_keys: vec![
                alice_pubkey,
                Pubkey::from_str("FCPPrgV66xj2uvegqgEVJ6cqkKNULKsMXxkSpzhauqdA").unwrap(),
                Pubkey::from_str("magyVRKhxESvpzvQd4qEc4dZfv8e9u5zTMam3BSk22T").unwrap(),
                Pubkey::from_str("8RjnD8Jy6A88WmfqXpGda7tvRzczqoNyA3usCxtME51a").unwrap(),
                Pubkey::from_str("7XLWyPdHWK8Fs6s1yzWnheFS61e2C6CUP7oTYH5VW34n").unwrap(),
                Pubkey::from_str("4Sz4W2pC1YaLZyVP6ptNXNf727c6BtnB5BEYNQhHdCxN").unwrap(),
                Pubkey::from_str("p9c32PDrUYuLvy9MsfmWa4ALUdUE7oaRAKmg6URmuR6").unwrap(),
                Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
                Pubkey::from_str("GmgkeeJtcjHgeiSDdT5gxznUDr5ygq9jo8tmA4ny7ziv").unwrap(),
                Pubkey::default(),
                Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
            ],
            address_table_lookups: vec![MessageAddressTableLookup {
                account_key: Pubkey::from_str("CCFK1x9aUeHoeRvbo87iq52NcVbz4Ff1cpfMScKZy4fy")
                    .unwrap(),
                writable_indexes: vec![57, 58, 59],
                readonly_indexes: vec![0, 60],
            }],
            instructions: versioned_transaction.message.instructions().to_vec(),
        });

        let sdk_message_data_bs58 = bs58::encode(version_0_message.serialize()).into_string();

        assert_eq!(message_data_bs58, sdk_message_data_bs58);
    }
}
