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
    let recent_blockhash = Hash::new_unique();

    (alice_pubkey, recent_blockhash)
}

pub fn get_transfer_transaction_string(maybe_recent_blockhash: Option<Hash>) -> String {
    let (alice_pubkey, new_recent_blockhash) = get_default_setup();
    let recent_blockhash = maybe_recent_blockhash.unwrap_or(new_recent_blockhash);

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

pub fn get_transfer_transaction_v0_string(maybe_recent_blockhash: Option<Hash>) -> String {
    let (alice_pubkey, new_recent_blockhash) = get_default_setup();
    let recent_blockhash = maybe_recent_blockhash.unwrap_or(new_recent_blockhash);

    json!({
      "signatures": [
        {
          "0": 0,
          "1": 0,
          "2": 0,
          "3": 0,
          "4": 0,
          "5": 0,
          "6": 0,
          "7": 0,
          "8": 0,
          "9": 0,
          "10": 0,
          "11": 0,
          "12": 0,
          "13": 0,
          "14": 0,
          "15": 0,
          "16": 0,
          "17": 0,
          "18": 0,
          "19": 0,
          "20": 0,
          "21": 0,
          "22": 0,
          "23": 0,
          "24": 0,
          "25": 0,
          "26": 0,
          "27": 0,
          "28": 0,
          "29": 0,
          "30": 0,
          "31": 0,
          "32": 0,
          "33": 0,
          "34": 0,
          "35": 0,
          "36": 0,
          "37": 0,
          "38": 0,
          "39": 0,
          "40": 0,
          "41": 0,
          "42": 0,
          "43": 0,
          "44": 0,
          "45": 0,
          "46": 0,
          "47": 0,
          "48": 0,
          "49": 0,
          "50": 0,
          "51": 0,
          "52": 0,
          "53": 0,
          "54": 0,
          "55": 0,
          "56": 0,
          "57": 0,
          "58": 0,
          "59": 0,
          "60": 0,
          "61": 0,
          "62": 0,
          "63": 0
        }
      ],
      "message": {
        "header": {
          "numRequiredSignatures": 1,
          "numReadonlySignedAccounts": 0,
          "numReadonlyUnsignedAccounts": 1
        },
        "staticAccountKeys": [
          alice_pubkey.to_string(),
          "11111111111111111111111111111111"
        ],
        "recentBlockhash": recent_blockhash.to_string(),
        "compiledInstructions": [
          {
            "programIdIndex": 1,
            "accountKeyIndexes": [
              0,
              0
            ],
            "data": {
              "type": "Buffer",
              "data": [
                2,
                0,
                0,
                0,
                0,
                152,
                13,
                0,
                0,
                0,
                0,
                0
              ]
            }
          }
        ],
        "addressTableLookups": []
      }
    })
    .to_string()
}
