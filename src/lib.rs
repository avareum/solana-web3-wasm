#![recursion_limit = "256"]

pub use solana_client_wasm;
pub use solana_extra_wasm;
pub use solana_sdk;

#[cfg(feature = "transaction_builder")]
pub use spl_token;

pub mod core;
pub mod info;
pub mod wasm;

#[cfg(feature = "tests")]
pub mod tests;

#[cfg(feature = "wallet_info")]
pub mod wallet;

#[cfg(feature = "nft_info")]
pub mod nft;

#[cfg(feature = "transaction_builder")]
pub mod transaction_builder;
