use std::collections::HashMap;

use crate::core::hash::{hash_deserialize, hash_serialize};
use crate::core::pubkey::{multiple_pubkey_deserialize, multiple_pubkey_serialize};
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
    #[serde(
        serialize_with = "hash_serialize",
        deserialize_with = "hash_deserialize"
    )]
    recent_blockhash: Hash,
    compiled_instructions: Vec<CompiledInstructionValue>,
    address_table_lookups: Vec<v0::MessageAddressTableLookup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledInstructionValue {
    program_id_index: u8,
    account_key_indexes: Vec<u8>,
    data: CompiledInstructionDataValue,
}

impl From<CompiledInstructionValue> for CompiledInstruction {
    fn from(value: CompiledInstructionValue) -> Self {
        CompiledInstruction::new(
            value.program_id_index,
            &value.data,
            value.account_key_indexes,
        )
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledInstructionDataValue {
    r#type: String,
    data: Vec<u8>,
}

impl From<TransactionV0MessageValue> for VersionedMessage {
    fn from(value: TransactionV0MessageValue) -> Self {
        VersionedMessage::V0(v0::Message {
            header: value.header,
            account_keys: value.static_account_keys,
            recent_blockhash: value.recent_blockhash,
            instructions: value
                .compiled_instructions
                .into_iter()
                .map(CompiledInstruction::from)
                .collect(),
            address_table_lookups: value.address_table_lookups,
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionV0Value {
    pub signatures: Vec<HashMap<String, Value>>,
    pub message: TransactionV0MessageValue,
}

impl From<TransactionV0Value> for VersionedTransaction {
    fn from(value: TransactionV0Value) -> Self {
        VersionedTransaction {
            signatures: value
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
            message: VersionedMessage::from(value.message),
        }
    }
}
