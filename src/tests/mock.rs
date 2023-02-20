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
      "feePayer": null,
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
                100,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ]
            }
          }
        ],
        "addressTableLookups": []
      }
    })
    .to_string()
}

pub fn get_swap_transactions_v0_with_address_table_lookups_string(
    maybe_recent_blockhash: Option<Hash>,
) -> Vec<String> {
    let (alice_pubkey, new_recent_blockhash) = get_default_setup();
    let recent_blockhash = maybe_recent_blockhash.unwrap_or(new_recent_blockhash);

    vec![
        json!({
          "recentBlockhash": recent_blockhash.to_string(),
          "feePayer": null,
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
                  "pubkey": "magyVRKhxESvpzvQd4qEc4dZfv8e9u5zTMam3BSk22T",
                  "isSigner": false,
                  "isWritable": true
                },
                {
                  "pubkey": alice_pubkey.to_string(),
                  "isSigner": false,
                  "isWritable": false
                },
                {
                  "pubkey": "So11111111111111111111111111111111111111112",
                  "isSigner": false,
                  "isWritable": false
                },
                {
                  "pubkey": "11111111111111111111111111111111",
                  "isSigner": false,
                  "isWritable": false
                },
                {
                  "pubkey": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                  "isSigner": false,
                  "isWritable": false
                },
                {
                  "pubkey": "SysvarRent111111111111111111111111111111111",
                  "isSigner": false,
                  "isWritable": false
                }
              ],
              "programId": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
              "data": [
                1
              ]
            }
          ],
          "signers": [
            alice_pubkey.to_string()
          ]
        })
        .to_string(),
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
              },
              {
                "0": 184,
                "1": 96,
                "2": 45,
                "3": 245,
                "4": 35,
                "5": 167,
                "6": 75,
                "7": 189,
                "8": 170,
                "9": 7,
                "10": 243,
                "11": 239,
                "12": 181,
                "13": 199,
                "14": 25,
                "15": 11,
                "16": 49,
                "17": 9,
                "18": 155,
                "19": 95,
                "20": 232,
                "21": 137,
                "22": 116,
                "23": 112,
                "24": 145,
                "25": 193,
                "26": 83,
                "27": 209,
                "28": 185,
                "29": 197,
                "30": 52,
                "31": 152,
                "32": 7,
                "33": 162,
                "34": 34,
                "35": 24,
                "36": 222,
                "37": 95,
                "38": 134,
                "39": 119,
                "40": 56,
                "41": 251,
                "42": 166,
                "43": 13,
                "44": 28,
                "45": 98,
                "46": 62,
                "47": 163,
                "48": 179,
                "49": 156,
                "50": 28,
                "51": 124,
                "52": 244,
                "53": 22,
                "54": 59,
                "55": 3,
                "56": 145,
                "57": 97,
                "58": 200,
                "59": 214,
                "60": 165,
                "61": 191,
                "62": 138,
                "63": 1
              }
            ],
            "message": {
              "header": {
                "numRequiredSignatures": 2,
                "numReadonlySignedAccounts": 0,
                "numReadonlyUnsignedAccounts": 4
              },
              "staticAccountKeys": [
                alice_pubkey.to_string(),
                "FCPPrgV66xj2uvegqgEVJ6cqkKNULKsMXxkSpzhauqdA",
                "magyVRKhxESvpzvQd4qEc4dZfv8e9u5zTMam3BSk22T",
                "8RjnD8Jy6A88WmfqXpGda7tvRzczqoNyA3usCxtME51a",
                "7XLWyPdHWK8Fs6s1yzWnheFS61e2C6CUP7oTYH5VW34n",
                "4Sz4W2pC1YaLZyVP6ptNXNf727c6BtnB5BEYNQhHdCxN",
                "p9c32PDrUYuLvy9MsfmWa4ALUdUE7oaRAKmg6URmuR6",
                "ComputeBudget111111111111111111111111111111",
                "GmgkeeJtcjHgeiSDdT5gxznUDr5ygq9jo8tmA4ny7ziv",
                "11111111111111111111111111111111",
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
              ],
              "recentBlockhash": recent_blockhash.to_string(),
              "compiledInstructions": [
                {
                  "programIdIndex": 7,
                  "accountKeyIndexes": [],
                  "data": {
                    "type": "Buffer",
                    "data": [
                      2,
                      192,
                      92,
                      21,
                      0
                    ]
                  }
                },
                {
                  "programIdIndex": 8,
                  "accountKeyIndexes": [
                    1,
                    0,
                    2,
                    9
                  ],
                  "data": {
                    "type": "Buffer",
                    "data": [
                      46,
                      138,
                      52,
                      163,
                      82,
                      125,
                      83,
                      166
                    ]
                  }
                },
                {
                  "programIdIndex": 8,
                  "accountKeyIndexes": [
                    1,
                    14,
                    2,
                    10,
                    0,
                    11,
                    2,
                    12,
                    3,
                    13,
                    4,
                    5,
                    6,
                    15
                  ],
                  "data": {
                    "type": "Buffer",
                    "data": [
                      28,
                      106,
                      166,
                      25,
                      217,
                      1,
                      229,
                      65,
                      64,
                      66,
                      15,
                      0,
                      0,
                      0,
                      0,
                      0,
                      8,
                      42,
                      0,
                      0,
                      0,
                      248,
                      198,
                      158,
                      145,
                      225,
                      117,
                      135,
                      200,
                      64,
                      66,
                      15,
                      0,
                      0,
                      0,
                      0,
                      0,
                      0,
                      0,
                      0,
                      0,
                      0,
                      0,
                      0,
                      0,
                      175,
                      51,
                      27,
                      168,
                      50,
                      127,
                      187,
                      53,
                      177,
                      196,
                      254,
                      255,
                      0,
                      0,
                      0,
                      0,
                      1,
                      0
                    ]
                  }
                },
                {
                  "programIdIndex": 8,
                  "accountKeyIndexes": [
                    0,
                    1,
                    2
                  ],
                  "data": {
                    "type": "Buffer",
                    "data": [
                      124,
                      51,
                      114,
                      144,
                      6,
                      33,
                      198,
                      211,
                      223,
                      149,
                      98,
                      2,
                      0,
                      0,
                      0,
                      0
                    ]
                  }
                },
                {
                  "programIdIndex": 10,
                  "accountKeyIndexes": [
                    2,
                    0,
                    0
                  ],
                  "data": {
                    "type": "Buffer",
                    "data": [
                      9
                    ]
                  }
                }
              ],
              "addressTableLookups": [
                {
                  "accountKey": "CCFK1x9aUeHoeRvbo87iq52NcVbz4Ff1cpfMScKZy4fy",
                  "writableIndexes": [
                    57,
                    58,
                    59
                  ],
                  "readonlyIndexes": [
                    0,
                    60
                  ]
                }
              ]
            }
          }
        )
        .to_string(),
    ]
}

pub fn get_tulip_vault_transactions_string(maybe_recent_blockhash: Option<Hash>) -> String {
    let (alice_pubkey, new_recent_blockhash) = get_default_setup();
    let recent_blockhash = maybe_recent_blockhash.unwrap_or(new_recent_blockhash);

    json!({
          "recentBlockhash": recent_blockhash.to_string(),
      "feePayer": null,
      "nonceInfo": null,
      "instructions": [
        {
          "keys": [
            {
              "pubkey": alice_pubkey.to_string(),
              "isSigner": true,
              "isWritable": false
            },
            {
              "pubkey": "3wPiV9inTGexMZjp6x5Amqwp2sRNtpSheG8Hbv2rgq8W",
              "isSigner": false,
              "isWritable": false
            },
            {
              "pubkey": "5RBtEnNYEVNRFfQsnudSFkC5cnRQMKTWp3fekfVmJEpD",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "2aEH7K3tJUL2fwR1se16HndTLUWYcKQvL3qj7G7uaYu1",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "9zFiwnpYEYC9tNFq5rSn54tmWYDX6dRyys9YT9TYpws2",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "Cvvh8nsKZet59nsDDo3orMa3rZnPWQhpgrMCVcRDRgip",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "ykxD9jdJvKK8F1CnDZrnfEDz6zCC886VrwBvVgdHrCd",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
              "isSigner": false,
              "isWritable": false
            },
            {
              "pubkey": "SysvarRent111111111111111111111111111111111",
              "isSigner": false,
              "isWritable": false
            },
            {
              "pubkey": "11111111111111111111111111111111",
              "isSigner": false,
              "isWritable": false
            }
          ],
          "programId": "TLPv2tuSVvn3fSk8RgW3yPddkp5oFivzZV3rA9hQxtX",
          "data": [
            55,
            114,
            97,
            238,
            33,
            173,
            193,
            225,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            1,
            0,
            0,
            0,
            0,
            0
          ]
        },
        {
          "keys": [
            {
              "pubkey": alice_pubkey.to_string(),
              "isSigner": true,
              "isWritable": true
            },
            {
              "pubkey": "9zFiwnpYEYC9tNFq5rSn54tmWYDX6dRyys9YT9TYpws2",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "ykxD9jdJvKK8F1CnDZrnfEDz6zCC886VrwBvVgdHrCd",
              "isSigner": false,
              "isWritable": false
            },
            {
              "pubkey": "Cvvh8nsKZet59nsDDo3orMa3rZnPWQhpgrMCVcRDRgip",
              "isSigner": false,
              "isWritable": false
            },
            {
              "pubkey": "11111111111111111111111111111111",
              "isSigner": false,
              "isWritable": false
            },
            {
              "pubkey": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
              "isSigner": false,
              "isWritable": false
            },
            {
              "pubkey": "SysvarRent111111111111111111111111111111111",
              "isSigner": false,
              "isWritable": false
            }
          ],
          "programId": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
          "data": []
        },
        {
          "keys": [
            {
              "pubkey": alice_pubkey.to_string(),
              "isSigner": true,
              "isWritable": false
            },
            {
              "pubkey": "3wPiV9inTGexMZjp6x5Amqwp2sRNtpSheG8Hbv2rgq8W",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "5RBtEnNYEVNRFfQsnudSFkC5cnRQMKTWp3fekfVmJEpD",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "ykxD9jdJvKK8F1CnDZrnfEDz6zCC886VrwBvVgdHrCd",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "14fdy6YXbhDgnVQz4VcgSGgUcZ35eE48SKDrfqF87NUP",
              "isSigner": false,
              "isWritable": false
            },
            {
              "pubkey": "36KtHLHxcGnrfEb2GLwPcbN9nHUkeoi3gd6rMQj8wwVj",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "Cvvh8nsKZet59nsDDo3orMa3rZnPWQhpgrMCVcRDRgip",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "9zFiwnpYEYC9tNFq5rSn54tmWYDX6dRyys9YT9TYpws2",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "B8MA5aWJ7xv3SQgmnLe5orh7zDt8ah6JybBsTPhkT1Ng",
              "isSigner": false,
              "isWritable": true
            },
            {
              "pubkey": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
              "isSigner": false,
              "isWritable": false
            }
          ],
          "programId": "TLPv2tuSVvn3fSk8RgW3yPddkp5oFivzZV3rA9hQxtX",
          "data": [
            110,
            72,
            179,
            47,
            131,
            109,
            115,
            103,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            16,
            39,
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
