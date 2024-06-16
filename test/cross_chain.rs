#![cfg(not(target_arch = "wasm32"))]

use linera_sdk::base::{Amount, Owner};
use linera_sdk::test::TestValidator;

#[tokio::test]
async fn test_cross_chain_transfer() {
    let initial_amount = Amount::from(1_000_000u128);
    let transfer_amount = Amount::from(50_000u128);

    let (validator, bytecode_id) = TestValidator::with_current_bytecode().await;
    let mut sender_chain = validator.new_chain().await;
    let sender_account = Owner::from(sender_chain.public.key());
    panic!()
}
