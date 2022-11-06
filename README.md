# solana-web3-wasm

Solana Wasm Web3 with handy utils

## Features

### Web3

- [x] Able to use `WasmClient` via `Web3Provider`.

### Tokens

- [x] Able to `get_multiple_token_metadata` via `metaplex`.
- [x] Able to `get_mint_metadata_map` via `metaplex`.
- [x] Able to `get_multiple_token_amount`.
- [x] Able to `get_and_deserialize_multiple_accounts_data`.
- [x] Able to `get_logo_by_mint_address` via `Raydium`.

## TODO

- [ ] Able to connect `Phantom`.
- [ ] Able to `signMessage` with `Phantom`.
- [ ] Able to `signTransaction` with `Phantom`.
- [ ] Able to `signAndSendTransaction` with `Phantom`.

## TOHAVE

- [ ] Add `test_get_and_deserialize_account_data` test.
- [ ] Support token logo that not exists via `Raydium`.
