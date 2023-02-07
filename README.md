# solana-web3-wasm

Wrapped `WasmClient` with handy features.

## Features

### Web3

- [ ] Support `message_data`.
  - [x] `connect`.
  - [x] `disconnect`.
  - [x] `signMessage`.
  - [x] `signTransaction`.
  - [x] `signAllTransactions`.
  - [ ] `signAndSendTransactionV0WithLookupTable`.
- [ ] Support `signatures`.

### Tokens

- [x] Able to `get_multiple_token_metadata` via `metaplex`.
- [x] Able to `get_mint_metadata_map` via `metaplex`.
- [x] Able to `get_multiple_token_amount`.
- [x] Able to `get_and_deserialize_multiple_accounts_data`.
- [x] Able to `get_logo_by_mint_address` via `Raydium`.

## TODO

- [ ] Able to test on some `client`.
- [ ] Able to get [The Fungible Asset](https://docs.metaplex.com/programs/token-metadata/token-standard#the-fungible-asset-standard)

## TOHAVE

- [ ] Add `test_get_and_deserialize_account_data` test.
- [ ] Support token logo that not exists via `Raydium`.
