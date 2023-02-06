# solana-web3-wasm

Wrapped `WasmClient` with handy features.

## Features

### Web3

- [x] Able to use as `phantom` alias.
- [ ] Able to prepare `message_data` for sign.
  - [x] `connect`.
  - [x] `disconnect`.
  - [ ] `signTransaction`.
  - [ ] `signAllTransactions`.
  - [ ] `signAndSendTransactionV0WithLookupTable`.
  - [ ] `signMessage`.

### Tokens

- [x] Able to `get_multiple_token_metadata` via `metaplex`.
- [x] Able to `get_mint_metadata_map` via `metaplex`.
- [x] Able to `get_multiple_token_amount`.
- [x] Able to `get_and_deserialize_multiple_accounts_data`.
- [x] Able to `get_logo_by_mint_address` via `Raydium`.

## TODO

- [ ] Complete `success_transfer_spl_no_ata`.
- [ ] Complete `success_test_transaction_from_string`.
- [ ] Complete `success_test_transactions_from_string`.
- [ ] Able to get [The Fungible Asset](https://docs.metaplex.com/programs/token-metadata/token-standard#the-fungible-asset-standard)
- [ ] Able to connect `Phantom`.
- [ ] Able to `signMessage` with `Phantom`.
- [ ] Able to `signTransaction` with `Phantom`.
- [ ] Able to `signAndSendTransaction` with `Phantom`.

## TOHAVE

- [ ] Support parse `Transaction` signatures. // Not require atm because we will sign with wallet, no pre-signed from `dApp`.
- [ ] Add `test_get_and_deserialize_account_data` test.
- [ ] Support token logo that not exists via `Raydium`.
