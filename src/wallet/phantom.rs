use std::collections::HashMap;

use serde::Deserialize;
use serde_json::{json, Value};
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
            let tx = Transaction::from(tx_value);
            tx.message_data()
        }
        // V0
        Err(_) => {
            let tx_json: TransactionV0Value = serde_json::from_str(tx)?;
            let tx = VersionedTransaction::from(tx_json);
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
    use super::*;
    use crate::tests::mock::*;
    use solana_sdk::{system_instruction, transaction::Transaction};

    #[tokio::test]
    async fn success_get_message_data_bs58_from_transaction() {
        // Setup
        let tx = get_transfer_transaction_string();
        let message_data_bs58 = get_message_data_bs58_from_transaction(tx.as_str()).unwrap();

        dbg!(&message_data_bs58);

        // Prove
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        let mut tx = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        tx.message.recent_blockhash = recent_blockhash;

        let message_data = tx.message_data();
        let sdk_message_data_bs58 = bs58::encode(message_data).into_string();

        dbg!(&sdk_message_data_bs58);

        assert_eq!(message_data_bs58, sdk_message_data_bs58);
    }

    #[tokio::test]
    async fn success_get_message_data_bs58_from_transactions() {
        // Setup
        let tx1_string = get_transfer_transaction_string();
        let tx2_string = get_transfer_transaction_string();
        let txs = vec![tx1_string, tx2_string];

        let message_data_bs58s = get_message_data_bs58_from_transactions(txs).unwrap();

        dbg!(&message_data_bs58s);

        // Prove
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        let mut tx = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        tx.message.recent_blockhash = recent_blockhash;

        let message_datas = vec![tx.message_data(), tx.message_data()];
        let sdk_message_data_bs58s = message_datas
            .iter()
            .map(|e| bs58::encode(e).into_string())
            .collect::<Vec<_>>();

        dbg!(&sdk_message_data_bs58s);

        assert_eq!(message_data_bs58s, sdk_message_data_bs58s);
    }

    // #[tokio::test]
    // async fn test_message_from_string() {
    //     let alice_pubkey = get_alice_keypair().pubkey();
    //     let recent_blockhash = Hash::new_from_array(
    //         Pubkey::from_str("9zb7KBbBo8brCsfMNe9dZhPcohiMVd8LPDJwHa82iNV1")
    //             .unwrap()
    //             .to_bytes(),
    //     );
    //     let message_data_bs58 = [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];

    //     todo!()
    // }

    #[tokio::test]
    async fn success_get_message_data_bs58_from_transaction_v0() {
        // Setup
        let tx = get_transfer_transaction_v0_string();
        let message_data_bs58 = get_message_data_bs58_from_transaction(tx.as_str()).unwrap();

        dbg!(&message_data_bs58);

        // // Prove
        // let (alice_pubkey, recent_blockhash) = get_default_setup();
        // let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        // let mut tx = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        // tx.message.recent_blockhash = recent_blockhash;

        // let message_data = tx.message_data();
        // let sdk_message_data_bs58 = bs58::encode(message_data).into_string();

        // dbg!(&sdk_message_data_bs58);

        // assert_eq!(message_data_bs58, sdk_message_data_bs58);
    }
}
