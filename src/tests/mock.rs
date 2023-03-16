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

pub fn get_kyber_transaction_string(_maybe_recent_blockhash: Option<Hash>) -> String {
    // let (alice_pubkey, new_recent_blockhash) = get_default_setup();
    // let recent_blockhash = maybe_recent_blockhash.unwrap_or(new_recent_blockhash);

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
          "numReadonlyUnsignedAccounts": 7
        },
        "staticAccountKeys": [
          "DcJGXTE7L1XQtFSdvBv2NPkGCxQ1cziem1yXnqfy2rVy",
          "Btq4pb11PfiWSc4bT7sjJXVFRTdZfTmimGQ7YY31S1AY",
          "Eja56YAPbhWM8kKKexjbYy2CmEE2RARP7Nw13ANNUHvS",
          "B8MA5aWJ7xv3SQgmnLe5orh7zDt8ah6JybBsTPhkT1Ng",
          "ComputeBudget111111111111111111111111111111",
          "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
          "FoRGERiW7odcCBGU1bztZi16osPBHjxharvDathL5eds",
          "11111111111111111111111111111111",
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
          "So11111111111111111111111111111111111111112",
          "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB"
        ],
        "recentBlockhash": "E6zXTdL7oodNQsSKJMhaoSG4m9MQ5fT2RL17SzymvPqW",
        "compiledInstructions": [
          {
            "programIdIndex": 4,
            "accountKeyIndexes": [],
            "data": {
              "0": 2,
              "1": 192,
              "2": 92,
              "3": 21,
              "4": 0
            }
          },
          {
            "programIdIndex": 5,
            "accountKeyIndexes": [
              0,
              1,
              0,
              6,
              7,
              8
            ],
            "data": {}
          },
          {
            "programIdIndex": 5,
            "accountKeyIndexes": [
              0,
              2,
              0,
              9,
              7,
              8
            ],
            "data": {}
          },
          {
            "programIdIndex": 10,
            "accountKeyIndexes": [
              8,
              0,
              2,
              21,
              8,
              11,
              22,
              12,
              13,
              14,
              23,
              15,
              11,
              11,
              11,
              11,
              11,
              11,
              3,
              1,
              0,
              21,
              8,
              16,
              22,
              17,
              18,
              19,
              23,
              20,
              16,
              16,
              16,
              16,
              16,
              16,
              1,
              2,
              0
            ],
            "data": {
              "0": 229,
              "1": 23,
              "2": 203,
              "3": 151,
              "4": 122,
              "5": 227,
              "6": 173,
              "7": 42,
              "8": 0,
              "9": 2,
              "10": 0,
              "11": 0,
              "12": 0,
              "13": 2,
              "14": 7,
              "15": 2,
              "16": 7,
              "17": 232,
              "18": 3,
              "19": 0,
              "20": 0,
              "21": 0,
              "22": 0,
              "23": 0,
              "24": 0,
              "25": 242,
              "26": 149,
              "27": 0,
              "28": 0,
              "29": 0,
              "30": 0,
              "31": 0,
              "32": 0,
              "33": 50,
              "34": 0,
              "35": 0
            }
          },
          {
            "programIdIndex": 8,
            "accountKeyIndexes": [
              2,
              0,
              0
            ],
            "data": {
              "0": 9
            }
          }
        ],
        "addressTableLookups": [
          {
            "accountKey": "FgBUhuGZb1GvgECmRXTUknzfEz4ra7qgCFiRmzKugj7i",
            "writableIndexes": [
              152,
              154,
              155,
              156,
              158
            ],
            "readonlyIndexes": [
              0,
              3,
              22
            ]
          },
          {
            "accountKey": "7YjEM3LE7WRHmCaDBWJd7enAbTbXEdD92AiB6Rj4nbeV",
            "writableIndexes": [
              92,
              94,
              95,
              96,
              98
            ],
            "readonlyIndexes": []
          }
        ]
      }
    })
    .to_string()
}

pub fn get_ribbon_transaction_string(_maybe_recent_blockhash: Option<Hash>) -> String {
    json!({
        "recentBlockhash": "xhJi34Lyqvqouv4QbxHGfqJsJNR1fKu2PWJsyLjrA6E",
        "feePayer": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
        "nonceInfo": null,
        "instructions": [
            {
                "keys": [
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": true
                    },
                    {
                        "pubkey": "2ExFjREhXHhkNQh62TD5HSFHdgA1Fs8pn8XFfasVNCSg",
                        "isSigner": true,
                        "isWritable": true
                    }
                ],
                "programId": "11111111111111111111111111111111",
                "data": [
                    0,
                    0,
                    0,
                    0,
                    240,
                    29,
                    31,
                    0,
                    0,
                    0,
                    0,
                    0,
                    165,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    6,
                    221,
                    246,
                    225,
                    215,
                    101,
                    161,
                    147,
                    217,
                    203,
                    225,
                    70,
                    206,
                    235,
                    121,
                    172,
                    28,
                    180,
                    133,
                    237,
                    95,
                    91,
                    55,
                    145,
                    58,
                    140,
                    245,
                    133,
                    126,
                    255,
                    0,
                    169
                ]
            },
            {
                "keys": [
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": true
                    },
                    {
                        "pubkey": "2ExFjREhXHhkNQh62TD5HSFHdgA1Fs8pn8XFfasVNCSg",
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
                    64,
                    66,
                    15,
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
                        "pubkey": "2ExFjREhXHhkNQh62TD5HSFHdgA1Fs8pn8XFfasVNCSg",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "So11111111111111111111111111111111111111112",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "SysvarRent111111111111111111111111111111111",
                        "isSigner": false,
                        "isWritable": false
                    }
                ],
                "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "data": [
                    1
                ]
            },
            {
                "keys": [
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": true
                    },
                    {
                        "pubkey": "E9zDVD19c5Cix3fVDXJ8gSMwCGRvHvK5aW1Es4bHYNuT",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "6RGoidD6X3sghGrCCMPmBXBBoZg9KCw35G3LDxxxsGLp",
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
                        "pubkey": "VjdsdQWWNFUHY6QwU9iRv2QndCf3yufuskAzr12Ykws",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": false
                    },
                    {
                        "pubkey": "2ExFjREhXHhkNQh62TD5HSFHdgA1Fs8pn8XFfasVNCSg",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "E9zDVD19c5Cix3fVDXJ8gSMwCGRvHvK5aW1Es4bHYNuT",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "2YNj4egax5WV1zSgq9hwJFNzHSYZo2rU7S8BZuMdQMKW",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "3xaNwyF7qms5cmSoEemPfHQpwEGzTsKqfrc7uLyRuWpC",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "So11111111111111111111111111111111111111112",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "6RGoidD6X3sghGrCCMPmBXBBoZg9KCw35G3LDxxxsGLp",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "3ghzHYHC7nXx11DEq7aGNHwTyT3SWmCs1B2Ay7HB8ZCk",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "6Sv6D5RsQ6qRBDDoTdBUYBz3gpjnCeEB1V5hnJ1yqNvo",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "6ovMxMnGZhq3FvneugXkq2ggoZE3VuJHKifUSnCNU5Bf",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "AR638eeUYZfbsB48AuZmbrEYusAmccoE62tcUJNAzRug",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "yK4Qc3PWWX5iP4DX4jqKrhNj3FmJX7jHWbeFuoude1b",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "11111111111111111111111111111111",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "SysvarRent111111111111111111111111111111111",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "83vsgm8EhQwFBXEkBSfFyA3KrxFG6xKY9DFN7LZXe9dL",
                        "isSigner": false,
                        "isWritable": true
                    }
                ],
                "programId": "RBN2XNc6JQU6ewFp9TyPq6WznsvNuumzSJkor1nJFcz",
                "data": [
                    126,
                    224,
                    21,
                    255,
                    228,
                    53,
                    117,
                    33,
                    64,
                    66,
                    15,
                    0,
                    0,
                    0,
                    0,
                    0,
                    254
                ]
            }
        ],
        "signers": []
    })
    .to_string()
}

pub fn get_solend_transaction_string(_maybe_recent_blockhash: Option<Hash>) -> String {
    json!({
        "recentBlockhash": "3mKLGKidsTgJFdzEjDhyqyNDGpcEzfWdHXgugqC1H6LG",
        "feePayer": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
        "nonceInfo": null,
        "instructions": [
            {
                "keys": [],
                "programId": "ComputeBudget111111111111111111111111111111",
                "data": [
                    2,
                    224,
                    147,
                    4,
                    0
                ]
            },
            {
                "keys": [],
                "programId": "ComputeBudget111111111111111111111111111111",
                "data": [
                    3,
                    149,
                    117,
                    0,
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
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": true
                    },
                    {
                        "pubkey": "488LpE3xDWfsjVMKiikP8nW8ntfKBL7wjoe4nNmFeCaX",
                        "isSigner": false,
                        "isWritable": true
                    }
                ],
                "programId": "11111111111111111111111111111111",
                "data": [
                    3,
                    0,
                    0,
                    0,
                    142,
                    131,
                    83,
                    79,
                    26,
                    182,
                    128,
                    66,
                    71,
                    245,
                    189,
                    250,
                    72,
                    166,
                    80,
                    5,
                    155,
                    184,
                    253,
                    9,
                    4,
                    225,
                    47,
                    55,
                    254,
                    215,
                    163,
                    126,
                    54,
                    95,
                    184,
                    74,
                    32,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    55,
                    82,
                    67,
                    122,
                    56,
                    119,
                    98,
                    54,
                    87,
                    88,
                    120,
                    85,
                    104,
                    65,
                    105,
                    103,
                    111,
                    107,
                    57,
                    116,
                    116,
                    103,
                    114,
                    86,
                    103,
                    68,
                    70,
                    70,
                    70,
                    98,
                    105,
                    98,
                    192,
                    167,
                    151,
                    0,
                    0,
                    0,
                    0,
                    0,
                    20,
                    5,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    6,
                    155,
                    139,
                    152,
                    90,
                    171,
                    83,
                    42,
                    69,
                    9,
                    13,
                    232,
                    85,
                    127,
                    205,
                    220,
                    190,
                    108,
                    183,
                    239,
                    199,
                    58,
                    10,
                    101,
                    176,
                    111,
                    146,
                    3,
                    93,
                    183,
                    62,
                    236
                ]
            },
            {
                "keys": [
                    {
                        "pubkey": "488LpE3xDWfsjVMKiikP8nW8ntfKBL7wjoe4nNmFeCaX",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "7RCz8wb6WXxUhAigok9ttgrVgDFFFbibcirECzWSBauM",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": false
                    },
                    {
                        "pubkey": "SysvarRent111111111111111111111111111111111",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                        "isSigner": false,
                        "isWritable": false
                    }
                ],
                "programId": "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo",
                "data": [
                    6
                ]
            },
            {
                "keys": [
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": true
                    },
                    {
                        "pubkey": "H8dTyMH6mmZxi59YchWkmjgZc7LSMj6fenKk854zPP9x",
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
                    240,
                    209,
                    227,
                    4,
                    0,
                    0,
                    0,
                    0
                ]
            },
            {
                "keys": [
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": true
                    },
                    {
                        "pubkey": "H8dTyMH6mmZxi59YchWkmjgZc7LSMj6fenKk854zPP9x",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
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
                "data": []
            },
            {
                "keys": [
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": true
                    },
                    {
                        "pubkey": "8A1tQjR1JJeXyLjtHm1G36GeGTg9aJX6aWyB2jFxMQ8c",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "AVxnqyCameKsKTCGVKeyJMA7vjHnxJit6afC8AM9MdMj",
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
                        "pubkey": "H8dTyMH6mmZxi59YchWkmjgZc7LSMj6fenKk854zPP9x",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "8A1tQjR1JJeXyLjtHm1G36GeGTg9aJX6aWyB2jFxMQ8c",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "UTABCRXirrbpCNDogCoqEECtM3V44jXGCsK23ZepV3Z",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "5cSfC32xBUYqGfkURLGfANuK64naHmMp27jUT7LQSujY",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "AVxnqyCameKsKTCGVKeyJMA7vjHnxJit6afC8AM9MdMj",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "7RCz8wb6WXxUhAigok9ttgrVgDFFFbibcirECzWSBauM",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "55YceCDfyvdcPPozDiMeNp9TpwmL1hdoTEFw5BMNWbpf",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "9QqRewoWbePkSH919xXn826h67ea1EFAVXhTdiJArDnx",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "488LpE3xDWfsjVMKiikP8nW8ntfKBL7wjoe4nNmFeCaX",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": false
                    },
                    {
                        "pubkey": "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR",
                        "isSigner": false,
                        "isWritable": false
                    },
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": false
                    },
                    {
                        "pubkey": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                        "isSigner": false,
                        "isWritable": false
                    }
                ],
                "programId": "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo",
                "data": [
                    14,
                    0,
                    180,
                    196,
                    4,
                    0,
                    0,
                    0,
                    0
                ]
            },
            {
                "keys": [
                    {
                        "pubkey": "H8dTyMH6mmZxi59YchWkmjgZc7LSMj6fenKk854zPP9x",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": false,
                        "isWritable": true
                    },
                    {
                        "pubkey": "AbK3CgMqCS4s4hDN87ge5pXAbTZ3aY67jCgjRHzLrSi5",
                        "isSigner": true,
                        "isWritable": false
                    }
                ],
                "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "data": [
                    9
                ]
            }
        ],
        "signers": []
    })
    .to_string()
}
