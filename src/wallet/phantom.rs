use crate::core::hash::{hash_deserialize, hash_serialize};
use crate::core::pubkey::{
    multiple_pubkey_deserialize, multiple_pubkey_serialize, pubkey_deserialize, pubkey_serialize,
};

use serde::{Deserialize, Serialize};
use solana_sdk::{
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    transaction::Transaction,
};

use base64;

// Core -------------------------------------

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransactionValue {
    #[serde(
        serialize_with = "hash_serialize",
        deserialize_with = "hash_deserialize"
    )]
    pub recent_blockhash: Hash,
    #[serde(
        serialize_with = "pubkey_serialize",
        deserialize_with = "pubkey_deserialize"
    )]
    pub fee_payer: Pubkey,
    pub nonce_info: Option<()>,
    pub instructions: Vec<InstructionValue>,
    #[serde(
        serialize_with = "multiple_pubkey_serialize",
        deserialize_with = "multiple_pubkey_deserialize"
    )]
    pub signers: Vec<Pubkey>,
    // TODO: Decide to support signatures fro partial-sign from dApp.
    // #[serde(with = "short_vec")]
    // pub signatures: Vec<Signature>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMetaValue {
    #[serde(
        serialize_with = "pubkey_serialize",
        deserialize_with = "pubkey_deserialize"
    )]
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionValue {
    #[serde(
        serialize_with = "pubkey_serialize",
        deserialize_with = "pubkey_deserialize"
    )]
    pub program_id: Pubkey,
    #[serde(rename = "keys")]
    pub accounts: Vec<AccountMetaValue>,
    pub data: Vec<u8>,
}

// From -------------------------------------

impl From<AccountMetaValue> for AccountMeta {
    fn from(meta_value: AccountMetaValue) -> Self {
        AccountMeta {
            pubkey: meta_value.pubkey,
            is_signer: meta_value.is_signer,
            is_writable: meta_value.is_writable,
        }
    }
}

impl From<InstructionValue> for Instruction {
    fn from(instruction_value: InstructionValue) -> Self {
        Instruction {
            program_id: instruction_value.program_id,
            accounts: instruction_value
                .accounts
                .into_iter()
                .map(AccountMeta::from)
                .collect(),
            data: instruction_value.data,
        }
    }
}

impl From<TransactionValue> for Transaction {
    fn from(tx_value: TransactionValue) -> Self {
        let instructions: Vec<Instruction> = tx_value
            .instructions
            .into_iter()
            .map(Instruction::from)
            .collect();

        let mut tx = Transaction::new_with_payer(&instructions, Some(&tx_value.fee_payer));
        tx.message.recent_blockhash = tx_value.recent_blockhash;

        tx
    }
}

// Fun -------------------------------------

pub enum EncodingType {
    Base64,
    Base58,
}

pub fn get_base64_message_data_from_transaction(tx: &str) -> anyhow::Result<String> {
    get_message_data_from_transaction(tx, &EncodingType::Base64)
}

pub fn get_base64_message_data_from_transactions(txs: Vec<String>) -> anyhow::Result<Vec<String>> {
    get_message_data_from_transactions(txs, &EncodingType::Base64)
}

pub fn get_message_data_from_transaction(
    tx: &str,
    encoding_type: &EncodingType,
) -> anyhow::Result<String> {
    let tx_json = serde_json::from_str(tx)?;
    let tx_value = serde_json::from_value::<TransactionValue>(tx_json).unwrap();
    let tx = Transaction::from(tx_value);
    let message_data = tx.message_data();

    // Encode
    let message_data_string = match encoding_type {
        EncodingType::Base58 => bs58::encode(message_data).into_string(),
        _ => base64::encode(message_data),
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

    use serde_json::json;
    use solana_sdk::{
        hash::Hash, pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction,
        transaction::Transaction,
    };

    use super::*;

    fn get_alice_keypair() -> Keypair {
        Keypair::from_bytes(&[
            48, 83, 2, 1, 1, 48, 5, 6, 3, 43, 101, 112, 4, 34, 4, 32, 255, 101, 36, 24, 124, 23,
            167, 21, 132, 204, 155, 5, 185, 58, 121, 75, 156, 227, 116, 193, 215, 38, 142, 22, 8,
            14, 229, 239, 119, 93, 5, 218, 161, 35, 3, 33, 0, 36, 100, 158, 252, 33, 161, 97, 185,
            62, 89, 99,
        ])
        .unwrap()
    }

    fn get_default_setup() -> (Pubkey, Hash) {
        let alice_pubkey = get_alice_keypair().pubkey();
        let recent_blockhash = Hash::new_from_array(
            Pubkey::from_str("9zb7KBbBo8brCsfMNe9dZhPcohiMVd8LPDJwHa82iNV1")
                .unwrap()
                .to_bytes(),
        );

        (alice_pubkey, recent_blockhash)
    }

    fn get_transfer_transaction_string() -> String {
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        json!({
          "recentBlockhash": recent_blockhash.to_string(),
          "feePayer": alice_pubkey.to_string(),
          "nonceInfo": null,
          "instructions": [
            {
              "keys": [
                {
                  "pubkey": alice_pubkey.to_string(),
                  "isSigner": true,
                  "isWritable": true
                },
                {
                  "pubkey": alice_pubkey.to_string(),
                  "isSigner": false,
                  "isWritable": true
                }
              ],
              "programId": "11111111111111111111111111111111",
              "data": [
                2,
                0,
                0,
                0,
                100,
                0,
                0,
                0,
                0,
                0,
                0,
                0
              ]
            }
          ],
          "signers": [
              alice_pubkey.to_string()
          ]
        })
        .to_string()
    }

    #[tokio::test]
    async fn success_get_base64_message_data_from_transaction() {
        // Setup
        let tx = get_transfer_transaction_string();
        let message_data_base64 = get_base64_message_data_from_transaction(tx.as_str()).unwrap();

        dbg!(&message_data_base64);

        // Prove
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        let mut tx = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        tx.message.recent_blockhash = recent_blockhash;

        let message_data = tx.message_data();
        let sdk_message_data_base64 = base64::encode(message_data);

        dbg!(&sdk_message_data_base64);

        assert_eq!(message_data_base64, sdk_message_data_base64);
    }

    #[tokio::test]
    async fn success_get_base64_message_data_from_transactions() {
        // Setup
        let tx1_string = get_transfer_transaction_string();
        let tx2_string = get_transfer_transaction_string();
        let txs = vec![tx1_string, tx2_string];

        let message_data_base64s = get_base64_message_data_from_transactions(txs).unwrap();

        dbg!(&message_data_base64s);

        // Prove
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        let mut tx = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        tx.message.recent_blockhash = recent_blockhash;

        let message_datas = vec![tx.message_data(), tx.message_data()];
        let sdk_message_data_base64s = message_datas.iter().map(base64::encode).collect::<Vec<_>>();

        dbg!(&sdk_message_data_base64s);

        assert_eq!(message_data_base64s, sdk_message_data_base64s);
    }

    // #[tokio::test]
    // async fn test_message_from_string() {
    //     let alice_pubkey = get_alice_keypair().pubkey();
    //     let recent_blockhash = Hash::new_from_array(
    //         Pubkey::from_str("9zb7KBbBo8brCsfMNe9dZhPcohiMVd8LPDJwHa82iNV1")
    //             .unwrap()
    //             .to_bytes(),
    //     );
    //     let message_data_base64 = [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];

    //     todo!()
    // }
}
