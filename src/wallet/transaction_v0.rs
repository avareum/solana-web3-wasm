use std::collections::HashMap;

use crate::core::buffer::{
    get_u8s_from_option_json_stringify_uint8, hashmap_or_buffer_deserialize, BufferData, Uint8Data,
};
use crate::core::hash::{hash_deserialize, hash_serialize};
use crate::core::pubkey::{
    multiple_pubkey_deserialize, multiple_pubkey_serialize, pubkey_deserialize, pubkey_serialize,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

use thiserror::Error;

// Errors -------------------------------------

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TransactionV0ValueError {
    #[error("Expected u64")]
    ExpectedU64,
    #[error("Invalid CompiledInstruction")]
    InvalidCompiledInstruction,
    #[error("Invalid CompiledInstructionData")]
    InvalidCompiledInstructionData,
    #[error("Invalid MessageAddressTableLookup")]
    InvalidMessageAddressTableLookup,
}

// Core -------------------------------------

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
    address_table_lookups: Vec<MessageAddressTableLookupValue>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAddressTableLookupValue {
    #[serde(
        serialize_with = "pubkey_serialize",
        deserialize_with = "pubkey_deserialize"
    )]
    /// Address lookup table account key
    pub account_key: Pubkey,
    /// List of indexes used to load writable account addresses
    pub writable_indexes: Vec<u8>,
    /// List of indexes used to load readonly account addresses
    pub readonly_indexes: Vec<u8>,
}

impl TryFrom<MessageAddressTableLookupValue> for v0::MessageAddressTableLookup {
    type Error = TransactionV0ValueError;

    fn try_from(value: MessageAddressTableLookupValue) -> Result<Self, Self::Error> {
        let message_address_table_lookup = v0::MessageAddressTableLookup {
            account_key: value.account_key,
            writable_indexes: value.writable_indexes,
            readonly_indexes: value.readonly_indexes,
        };

        Ok(message_address_table_lookup)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledInstructionValue {
    program_id_index: u8,
    account_key_indexes: Vec<u8>,
    // #[serde(deserialize_with = "hashmap_or_buffer_deserialize")]
    data: Value,
}

impl TryFrom<CompiledInstructionValue> for CompiledInstruction {
    type Error = TransactionV0ValueError;

    fn try_from(value: CompiledInstructionValue) -> Result<Self, Self::Error> {
        let value_data = json! ({ "data": value.data });
        let data = serde_json::from_value::<Uint8Data>(value_data.clone());
        let data = match data {
            Ok(data) => data.data,
            Err(_) => {
                serde_json::from_value::<BufferData>(value_data)
                    .unwrap()
                    .data
            }
        };

        let compiled_tx = CompiledInstruction::new_from_raw_parts(
            value.program_id_index,
            data,
            value.account_key_indexes,
        );

        Ok(compiled_tx)
    }
}

impl TryFrom<TransactionV0MessageValue> for VersionedMessage {
    type Error = TransactionV0ValueError;

    fn try_from(value: TransactionV0MessageValue) -> Result<Self, Self::Error> {
        let instructions = value
            .compiled_instructions
            .into_iter()
            .map(CompiledInstruction::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| TransactionV0ValueError::InvalidCompiledInstruction)?;

        let address_table_lookups = value
            .address_table_lookups
            .into_iter()
            .map(v0::MessageAddressTableLookup::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| TransactionV0ValueError::InvalidMessageAddressTableLookup)?;

        let versioned_message = VersionedMessage::V0(v0::Message {
            header: value.header,
            account_keys: value.static_account_keys,
            recent_blockhash: value.recent_blockhash,
            instructions,
            address_table_lookups,
        });

        Ok(versioned_message)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionV0Value {
    pub signatures: Option<Vec<HashMap<String, Value>>>,
    pub message: TransactionV0MessageValue,
}

impl TryFrom<TransactionV0Value> for VersionedTransaction {
    type Error = TransactionV0ValueError;

    fn try_from(value: TransactionV0Value) -> Result<Self, Self::Error> {
        let signatures = get_u8s_from_option_json_stringify_uint8(value.signatures)
            .into_iter()
            .map(|e| Signature::new(&e))
            .collect::<Vec<_>>();

        let message = VersionedMessage::try_from(value.message)?;

        Ok(VersionedTransaction {
            signatures,
            message,
        })
    }
}
