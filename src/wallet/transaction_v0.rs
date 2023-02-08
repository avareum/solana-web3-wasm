use std::collections::HashMap;

use crate::core::hash::{hash_deserialize, hash_serialize};
use crate::core::pubkey::{
    multiple_pubkey_deserialize, multiple_pubkey_serialize, pubkey_deserialize, pubkey_serialize,
};
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
    data: CompiledInstructionDataValue,
}

impl TryFrom<CompiledInstructionValue> for CompiledInstruction {
    type Error = TransactionV0ValueError;

    fn try_from(value: CompiledInstructionValue) -> Result<Self, Self::Error> {
        if value.data.data.is_empty() {
            return Err(TransactionV0ValueError::InvalidCompiledInstructionData);
        }

        let compiled_tx = CompiledInstruction::new_from_raw_parts(
            value.program_id_index,
            value.data.data,
            value.account_key_indexes,
        );

        Ok(compiled_tx)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledInstructionDataValue {
    r#type: String,
    data: Vec<u8>,
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
        let signatures = match value.signatures {
            Some(signatures) => signatures
                .into_iter()
                .map(|s| {
                    let u8s = s
                        .into_values()
                        .map(|e| match e.as_u64() {
                            Some(num) => Ok(num as u8),
                            None => Err(TransactionV0ValueError::ExpectedU64),
                        })
                        .collect::<Result<Vec<u8>, TransactionV0ValueError>>()?;
                    Ok(Signature::new(&u8s))
                })
                .collect::<Result<Vec<Signature>, TransactionV0ValueError>>()?,
            None => vec![],
        };
        let message = VersionedMessage::try_from(value.message)?;

        Ok(VersionedTransaction {
            signatures,
            message,
        })
    }
}
