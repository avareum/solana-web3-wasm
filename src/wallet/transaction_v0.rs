use std::collections::HashMap;

use crate::core::pubkey::{multiple_pubkey_deserialize, multiple_pubkey_serialize};
use crate::core::signature::{multiple_signature_deserialize, multiple_signature_serialize};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_sdk::{
    hash::Hash,
    instruction::CompiledInstruction,
    message::{
        v0::{self},
        MessageHeader, VersionedMessage,
    },
    pubkey::Pubkey,
    signature::Signature,
    transaction::VersionedTransaction,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionV0MessageValue {
    header: MessageHeader,
    #[serde(
        serialize_with = "multiple_pubkey_serialize",
        deserialize_with = "multiple_pubkey_deserialize"
    )]
    static_account_keys: Vec<Pubkey>,
    recent_blockhash: Hash,
    compiled_instructions: Vec<CompiledInstruction>,
    address_table_lookups: Vec<v0::MessageAddressTableLookup>,
}

// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CompiledInstruction {
//     program_id_index: i64,
//     account_key_indexes: Vec<i64>,
//     data: CompiledInstructionData,
// }

// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CompiledInstructionData {
//     data_type: String,
//     data: Vec<i64>,
// }

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionV0ValueNotWorking {
    #[serde(
        serialize_with = "multiple_signature_serialize",
        deserialize_with = "multiple_signature_deserialize"
    )]
    signatures: Vec<Signature>,
    #[serde(skip_serializing)]
    message: TransactionV0MessageValue,
}

impl From<TransactionV0MessageValue> for VersionedMessage {
    fn from(message: TransactionV0MessageValue) -> Self {
        VersionedMessage::V0(v0::Message {
            header: message.header,
            account_keys: message.static_account_keys,
            recent_blockhash: message.recent_blockhash,
            instructions: message.compiled_instructions,
            address_table_lookups: message.address_table_lookups,
        })
    }
}

impl From<TransactionV0ValueNotWorking> for VersionedTransaction {
    fn from(tx_value: TransactionV0ValueNotWorking) -> Self {
        VersionedTransaction {
            signatures: vec![Signature::default()],
            message: VersionedMessage::from(tx_value.message),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionV0Value {
    pub signatures: Vec<HashMap<String, Value>>,
    // pub message: TransactionV0MessageValue,
}

impl From<TransactionV0Value> for VersionedTransaction {
    fn from(tx_value: TransactionV0Value) -> Self {
        VersionedTransaction {
            signatures: tx_value
                .signatures
                .into_iter()
                .map(|s| {
                    let u8s = s
                        .into_values()
                        .map(|e| e.as_u64().expect("expected valid value") as u8)
                        .collect::<Vec<u8>>();
                    Signature::new(&u8s)
                })
                .collect::<Vec<Signature>>(),
            message: VersionedMessage::default(),
        }
    }
}
