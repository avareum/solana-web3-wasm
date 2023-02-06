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
    // TODO: Decide to support signatures for partial-sign from dApp.
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
