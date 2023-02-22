use std::collections::HashMap;

use crate::core::buffer::get_u8s_from_option_json_stringify_uint8;
use crate::core::hash::{hash_deserialize, hash_serialize};
use crate::core::pubkey::{
    multiple_pubkey_deserialize, multiple_pubkey_serialize, option_pubkey_deserialize,
    option_pubkey_serialize, pubkey_deserialize, pubkey_serialize,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_sdk::signature::Signature;
use solana_sdk::{
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    transaction::Transaction,
};

use thiserror::Error;

// Errors -------------------------------------

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TransactionValueError {
    #[error("Expected u64")]
    ExpectedU64,
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Invalid AccountMeta")]
    InvalidAccountMeta,
}

// Core -------------------------------------

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionValue {
    #[serde(
        serialize_with = "hash_serialize",
        deserialize_with = "hash_deserialize"
    )]
    pub recent_blockhash: Hash,
    #[serde(
        serialize_with = "option_pubkey_serialize",
        deserialize_with = "option_pubkey_deserialize"
    )]
    pub fee_payer: Option<Pubkey>,
    pub nonce_info: Option<()>,
    pub instructions: Vec<InstructionValue>,
    #[serde(
        serialize_with = "multiple_pubkey_serialize",
        deserialize_with = "multiple_pubkey_deserialize"
    )]
    pub signers: Vec<Pubkey>,
    pub signatures: Option<Vec<HashMap<String, Value>>>,
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

impl TryFrom<AccountMetaValue> for AccountMeta {
    type Error = TransactionValueError;

    fn try_from(meta_value: AccountMetaValue) -> Result<Self, Self::Error> {
        Ok(AccountMeta {
            pubkey: meta_value.pubkey,
            is_signer: meta_value.is_signer,
            is_writable: meta_value.is_writable,
        })
    }
}

impl TryFrom<InstructionValue> for Instruction {
    type Error = TransactionValueError;

    fn try_from(instruction_value: InstructionValue) -> Result<Self, Self::Error> {
        if instruction_value.accounts.is_empty() {
            return Err(TransactionValueError::InvalidInstruction);
        }

        let accounts = instruction_value
            .accounts
            .into_iter()
            .map(AccountMeta::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let ix = Instruction {
            program_id: instruction_value.program_id,
            accounts,
            data: instruction_value.data,
        };

        Ok(ix)
    }
}

impl TryFrom<TransactionValue> for Transaction {
    type Error = TransactionValueError;

    fn try_from(value: TransactionValue) -> Result<Self, Self::Error> {
        let instructions: Vec<Instruction> = value
            .instructions
            .into_iter()
            .map(Instruction::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let mut tx = Transaction::new_with_payer(&instructions, value.fee_payer.as_ref());
        tx.message.recent_blockhash = value.recent_blockhash;

        tx.signatures = get_u8s_from_option_json_stringify_uint8(value.signatures)
            .into_iter()
            .map(|e| Signature::new(&e))
            .collect::<Vec<_>>();

        Ok(tx)
    }
}
