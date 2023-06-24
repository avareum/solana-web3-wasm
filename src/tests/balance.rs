#![cfg(test)]

use std::time::Duration;

use fluvio_wasm_timer::Delay;
use solana_client_wasm::WasmClient;
use solana_sdk::pubkey::Pubkey;

#[allow(dead_code)]
pub async fn wait_for_balance_change(client: &WasmClient, account: &Pubkey, balance_before: u64) {
    let mut i = 0;
    let max_loops = 60;
    loop {
        let balance_after = client.get_balance(account).await.unwrap();
        match balance_after.checked_sub(balance_before) {
            Some(0) => {
                Delay::new(Duration::from_secs(1)).await.ok();
                i += 1;
                dbg!(i);
            }
            Some(delta) => {
                assert!(delta != 0);
                break;
            }
            None => {
                // assert_eq!(balance_before - balance_after, expected_change);
                break;
            }
        }
        if i == max_loops {
            panic!("test was running for {} seconds", max_loops);
        }
    }
}
