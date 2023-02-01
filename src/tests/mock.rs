use std::str::FromStr;

use serde_json::json;
use solana_sdk::{hash::Hash, pubkey::Pubkey, signature::Keypair, signer::Signer};

#[allow(dead_code)]
#[rustfmt::skip]
pub const ALICE: &[u8] = &[
    57,99,241,156,126,127,97,60,
    40,14,39,4,115,72,39,75,
    2,14,30,255,45,79,195,202,
    132,18,131,180,61,12,87,183,
    14,175,192,115,62,33,136,190,
    244,254,192,174,2,126,227,113,
    222,42,224,89,36,89,239,167,
    22,150,31,29,89,188,176,162
];

#[allow(dead_code)]
#[rustfmt::skip]
pub const BOB: &[u8] = &[
    176,252,96,172,240,61,215,84,
    138,250,147,178,208,59,227,60,
    190,204,80,88,55,137,236,252,
    231,118,253,64,65,106,39,5,
    14,212,250,187,124,127,43,205,
    30,117,63,227,13,218,202,68,
    160,161,52,12,59,211,152,183,
    119,140,213,205,174,210,108,128
];

#[allow(dead_code)]
pub const AIRDROP_AMOUNT: u64 = 10000000; // tx free of 5000 lamports included

pub fn get_alice_keypair() -> Keypair {
    Keypair::from_bytes(ALICE).unwrap()
}

pub fn get_default_setup() -> (Pubkey, Hash) {
    let alice_pubkey = get_alice_keypair().pubkey();
    dbg!(alice_pubkey);

    let recent_blockhash = Hash::new_from_array(
        Pubkey::from_str("9zb7KBbBo8brCsfMNe9dZhPcohiMVd8LPDJwHa82iNV1")
            .unwrap()
            .to_bytes(),
    );

    (alice_pubkey, recent_blockhash)
}

pub fn get_transfer_transaction_string() -> String {
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
