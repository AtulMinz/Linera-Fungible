#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::str::FromStr;

use linera_sdk::{
    base::{Amount, WithContractAbi},
    views::{RootView, View, ViewStorageContext},
    Contract, ContractRuntime,
};
use my_fungible::Message;

use self::state::Fungible;

pub struct FungibleContract {
    state: Fungible,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(FungibleContract);

impl WithContractAbi for FungibleContract {
    type Abi = my_fungible::ApplicationAbi;
}

impl Contract for FungibleContract {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Fungible::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        FungibleContract { state, runtime }
    }

    //The first time the application is created on a given chain.
    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        //Here we are creating a million tokens and giving it to the one initialising the operation.
        let amount = Amount::from_str("1_000_000").unwrap();
        if let Some(owner) = self.runtime.authenticated_signer() {
            self.state.initialize_accounts(owner, amount).await;
        }
    }

    //Happens when a block is created with a transaction which contains an operation.
    async fn execute_operation(&mut self, _operation: Self::Operation) -> Self::Response {}

    //How do we handle incoming message coming from a different chain.
    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
