use anyhow::bail;
use solana_sdk::transaction::{Transaction, VersionedTransaction};

#[cfg(feature = "wasm_bindgen")]
use wasm_bindgen::prelude::*;

use crate::wallet::transaction_v0::TransactionV0Value;

use super::transaction::TransactionValue;

// Type -------------------------------------

#[wasm_bindgen]
pub enum EncodingType {
    Base58,
    Base64,
}

// Fun -------------------------------------

pub fn get_message_data_bs58_from_string(tx_str: &str) -> anyhow::Result<String> {
    get_encoded_message_data_from_string(tx_str, &EncodingType::Base58)
}

pub fn get_multiple_message_data_bs58_from_string(
    tx_strs: Vec<String>,
) -> anyhow::Result<Vec<String>> {
    get_multiple_message_data_from_string(tx_strs, &EncodingType::Base58)
}

pub fn get_versioned_transaction_from_string(tx_str: &str) -> anyhow::Result<VersionedTransaction> {
    let tx_json = serde_json::from_str(tx_str)?;
    let tx_value = serde_json::from_value::<TransactionValue>(tx_json);

    Ok(match tx_value {
        // Legacy
        Ok(tx_value) => {
            let tx = Transaction::try_from(tx_value)?;
            VersionedTransaction::from(tx)
        }
        // V0
        Err(_) => {
            let tx_json_value = serde_json::from_str::<TransactionV0Value>(tx_str)?;
            VersionedTransaction::try_from(tx_json_value)?
        }
    })
}

pub fn get_encoded_message_data_from_string(
    tx_str: &str,
    encoding_type: &EncodingType,
) -> anyhow::Result<String> {
    // Parse transaction
    let tx = get_versioned_transaction_from_string(tx_str)?;
    let message_data = tx.message.serialize();

    // Encode
    let message_data_string = match encoding_type {
        EncodingType::Base58 => bs58::encode(message_data).into_string(),
        EncodingType::Base64 => base64::encode(message_data),
    };

    Ok(message_data_string)
}

pub fn get_multiple_message_data_from_string(
    txs: Vec<String>,
    encoding_type: &EncodingType,
) -> anyhow::Result<Vec<String>> {
    let mut errors = vec![];
    let result = txs
        .into_iter()
        .map(|e| get_encoded_message_data_from_string(&e, encoding_type))
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        bail!("errors: {:?}", errors)
    }

    Ok(result)
}

// Versioned Transaction -------------------------------------

pub fn get_encoded_versioned_transaction_from_string(
    tx_str: &str,
    encoding_type: &EncodingType,
) -> anyhow::Result<String> {
    // Parse transaction
    let tx = get_versioned_transaction_from_string(tx_str)?;
    let message_data = bincode::serialize(&tx)?;
    let message_data_string = match encoding_type {
        EncodingType::Base58 => bs58::encode(message_data).into_string(),
        EncodingType::Base64 => base64::encode(message_data),
    };

    Ok(message_data_string)
}

pub fn get_multiple_versioned_transactions_from_string(
    txs: Vec<String>,
    encoding_type: &EncodingType,
) -> anyhow::Result<Vec<String>> {
    let mut errors = vec![];
    let result = txs
        .into_iter()
        .map(|e| get_encoded_versioned_transaction_from_string(&e, encoding_type))
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        bail!("errors: {:?}", errors)
    }

    Ok(result)
}

pub fn get_bs58_multiple_versioned_transactions_from_string(
    txs: Vec<String>,
) -> anyhow::Result<Vec<String>> {
    get_multiple_versioned_transactions_from_string(txs, &EncodingType::Base58)
}

// Test -------------------------------------

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {

    use super::*;
    use crate::tests::mock::*;
    use solana_sdk::{
        instruction::CompiledInstruction,
        message::{v0, MessageHeader, VersionedMessage},
        pubkey::Pubkey,
        system_instruction,
        transaction::Transaction,
    };

    #[tokio::test]
    async fn success_legacy_get_message_data_bs58_from_string() {
        // Setup
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let tx = get_transfer_transaction_string(Some(recent_blockhash));
        let message_data_bs58 = get_message_data_bs58_from_string(tx.as_str()).unwrap();

        // Prove
        let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        let mut tx = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        tx.message.recent_blockhash = recent_blockhash;

        let message_data = tx.message_data();
        let sdk_message_data_bs58 = bs58::encode(message_data).into_string();

        assert_eq!(message_data_bs58, sdk_message_data_bs58);
    }

    #[tokio::test]
    async fn success_legacy_get_multiple_message_data_bs58_from_string() {
        // Setup
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let tx1_string = get_transfer_transaction_string(Some(recent_blockhash));
        let tx2_string = get_transfer_transaction_string(Some(recent_blockhash));
        let txs = vec![tx1_string, tx2_string];

        let message_data_bs58s = get_multiple_message_data_bs58_from_string(txs).unwrap();

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
    async fn success_v0_get_message_data_bs58_from_string() {
        // Setup
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let mocked_tx_v0 = get_transfer_transaction_v0_string(Some(recent_blockhash));
        let message_data_bs58 = get_message_data_bs58_from_string(mocked_tx_v0.as_str()).unwrap();

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
    async fn success_v0_get_multiple_message_data_bs58_from_string_with_address_table_lookups() {
        // Setup
        let (alice_pubkey, recent_blockhash) = get_default_setup();
        let mocked_txs_v0 =
            get_swap_transactions_v0_with_address_table_lookups_string(Some(recent_blockhash));

        let message_data_bs64 = get_encoded_versioned_transaction_from_string(
            mocked_txs_v0[0].as_str(),
            &EncodingType::Base64,
        );
        println!("{message_data_bs64:?}");

        let message_data_bs58 = get_encoded_versioned_transaction_from_string(
            mocked_txs_v0[0].as_str(),
            &EncodingType::Base58,
        );
        println!("{message_data_bs58:?}");

        let message_data_bs58s = get_multiple_message_data_bs58_from_string(mocked_txs_v0).unwrap();
        println!("{message_data_bs58s:#?}");

        // Prove tx0
        let ix = system_instruction::transfer(&alice_pubkey, &alice_pubkey, 100);
        let mut tx0 = Transaction::new_with_payer(&[ix], Some(&alice_pubkey));
        tx0.message.recent_blockhash = recent_blockhash;

        // Create v0 compatible message
        let alice_keypair = get_alice_keypair();
        let versioned_transaction0 = match VersionedTransaction::try_new(
            VersionedMessage::Legacy(tx0.message),
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
        println!("versioned_transaction0:{:#?}", versioned_transaction0);
        assert!(!versioned_transaction0.message.serialize().is_empty());

        // TODO: mock to matched tx1
        // // Prove tx1
        // let version_0_message = VersionedMessage::V0(v0::Message {
        //     header: MessageHeader {
        //         num_required_signatures: 2,
        //         num_readonly_signed_accounts: 0,
        //         num_readonly_unsigned_accounts: 4,
        //     },
        //     recent_blockhash,
        //     account_keys: vec![
        //         alice_pubkey,
        //         Pubkey::from_str("FCPPrgV66xj2uvegqgEVJ6cqkKNULKsMXxkSpzhauqdA").unwrap(),
        //         Pubkey::from_str("magyVRKhxESvpzvQd4qEc4dZfv8e9u5zTMam3BSk22T").unwrap(),
        //         Pubkey::from_str("8RjnD8Jy6A88WmfqXpGda7tvRzczqoNyA3usCxtME51a").unwrap(),
        //         Pubkey::from_str("7XLWyPdHWK8Fs6s1yzWnheFS61e2C6CUP7oTYH5VW34n").unwrap(),
        //         Pubkey::from_str("4Sz4W2pC1YaLZyVP6ptNXNf727c6BtnB5BEYNQhHdCxN").unwrap(),
        //         Pubkey::from_str("p9c32PDrUYuLvy9MsfmWa4ALUdUE7oaRAKmg6URmuR6").unwrap(),
        //         Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
        //         Pubkey::from_str("GmgkeeJtcjHgeiSDdT5gxznUDr5ygq9jo8tmA4ny7ziv").unwrap(),
        //         Pubkey::default(),
        //         Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        //     ],
        //     address_table_lookups: vec![MessageAddressTableLookup {
        //         account_key: Pubkey::from_str("CCFK1x9aUeHoeRvbo87iq52NcVbz4Ff1cpfMScKZy4fy")
        //             .unwrap(),
        //         writable_indexes: vec![57, 58, 59],
        //         readonly_indexes: vec![0, 60],
        //     }],
        //     instructions: versioned_transaction0.message.instructions().to_vec(),
        // });

        // use solana_sdk::signature::Signature;

        // let versioned_transaction1 = match VersionedTransaction::try_new(
        //     version_0_message,
        //     &[&alice_keypair],
        // ) {
        //     Ok(mut tx) => {
        //         tx.signatures = vec![
        //             solana_sdk::signature::Signature::default(),
        //             Signature::from_str("2gDpLGM7mtrVTZNKKnpnEWKvHmfE1SurpsXgktFXoifSdcGmiGjrdhnXrjTo7GNgaNQAgpREoUN9o69XVBKQCzUg").unwrap(),
        //             ];
        //         assert_eq!(tx.verify_with_results(), vec![true; 1]);
        //         tx
        //     }
        //     Err(err) => {
        //         dbg!(&err);
        //         assert_eq!(Some(err), None);
        //         panic!("error");
        //     }
        // };
        // println!("versioned_transaction1:{:#?}", versioned_transaction1);

        // let sdk_message_data_bs58_0 =
        //     bs58::encode(versioned_transaction0.message.serialize()).into_string();
        // let sdk_message_data_bs58_1 =
        //     bs58::encode(versioned_transaction1.message.serialize()).into_string();

        // println!("1️⃣ message_data_bs58s:{:#?}", message_data_bs58s);
        // println!(
        //     "2️⃣ sdk_message_data_bs58:{:#?}",
        //     [sdk_message_data_bs58_0, sdk_message_data_bs58_1]
        // );
        // assert_eq!(message_data_bs58s[0], sdk_message_data_bs58);

        assert_eq!(message_data_bs58s[0],"2TkSSmFj4tUyswyprMkfLq1phJqUAxkjBxwgUKyYyHjE3bWKhVaUX2hCaLtmDXpUufzaSkZ6g4JgeJw4HLs4Vp3EBsEEvkf9K9ZBUpRZmj6qSSVuvscvzDF4PMY7R6nARUaVmgQ9t94i2z3cbobfzNq9QH2P2C4pEmfCzLQMJ1cVJuwC87vimhTtnxc2WXQHp1kFW3WvadgJ13xREZLGxcJYucKjftnZ3r22dN8TqZWP9mpc8PFxWJfnV6sQDyZYjbWNsYQmWYPoAWrvKYPeuBZK6PKYcCg69gQsi1YmGUfAX1AkbsAXmEPykHPsXAqrTjDArU3oNpuzQ6L24ZhCzmN7tLeyneJbSePYNZ6fUmqBoPorUsS");
        assert_eq!(message_data_bs58s[1],"4ooqeY3sQsdzpv2X7wAReQakWSDKA5WMG5dJcQsw4fSUg5pNF8MyDT6URPyHqdVGzJiKkUuKQaPXsTLU3sqJBivv64aawUccq1zZ1hAq6GGzubdvsP5jNst2YbW8HKjWimG1ht2Ej2ASNtWi1DsckrYcurvD2gCF4mGNUoDbNUgW8q61QvQui7hmsQpT5phiF6h7ocRPHCd2S56oqSa35hd8bRVLSsPysbFJ8FHRzNx1FWKtt7yBK3UfaZBkKTebbcdsK1kKAGypz14tZLkXEsdJu25T2peHZuSHcyoCNDYqoxP9q8tzctGis6w7Hw1JuQ5EX2zjGNDthPuPKoHzYnED8SQovDR6GXdTD7DJh8sthSzrEGXtzzKxWLY5h1gVVJ59z7DeBuh52wNfc4v3kcdbc8CPrz9i1nq7geYVzgoMc9gni8w5mt26kyyHD6kkcAny5Ryp56M1ewVh9PgbvAXxXUgALohYosSTDGKmMgGjrQf2vMSJcS6FhQFVKTusCcbHbiqhjsVWWQLKFUBdqCDHt1DuU6wNQ4AZmKHMcDbzkQggV9GPN2Y9ty9QDXBs5kG4rdWngGQzACAzdGkCAuiBDwGJBQ3JJZdCYuZv4EXGn4CswN6gFxvqZxJHzriwfXs83E1uUy5MEpRVPV1YcCj95xwR3Zx9v458gmPBAi1wPvRBbY8dJ6y6WXPyAcHUo6qkNiUBwQ4bUiaXuJL3rUddHJpJUVZQzzfELGdC9FH19HPiXbSrwCFBGC4154qq");
    }

    #[tokio::test]
    async fn success_v0_get_bs58_multiple_versioned_transactions_from_string() {
        // Setup
        let (_, recent_blockhash) = get_default_setup();
        let mocked_txs_v0 =
            get_swap_transactions_v0_with_address_table_lookups_string(Some(recent_blockhash));

        let bs58_multiple_versioned_transactions =
            get_bs58_multiple_versioned_transactions_from_string(mocked_txs_v0).unwrap();

        println!(
            "bs58_multiple_versioned_transactions:{:#?}",
            bs58_multiple_versioned_transactions
        );
    }
}
