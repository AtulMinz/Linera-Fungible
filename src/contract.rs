#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::str::FromStr;

use linera_sdk::{
    base::{Amount, Owner, WithContractAbi},
    views::{RootView, View, ViewStorageContext},
    Contract, ContractRuntime,
};
use my_fungible::{Account, Message, Operation};

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
    type InstantiationArgument = Amount;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Fungible::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        FungibleContract { state, runtime }
    }

    //The first time the application is created on a given chain.
    async fn instantiate(&mut self, amount: Self::InstantiationArgument) {
        //Here we are creating a million tokens and giving it to the one initialising the operation.
        let amount = Amount::from_str("1_000_000").unwrap();
        if let Some(owner) = self.runtime.authenticated_signer() {
            self.state.initialize_accounts(owner, amount).await;
        }
    }

    //Happens when a block is created with a transaction which contains an operation.
    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::Transfer {
                owner,
                amount,
                target_account,
            } => {
                self.check_account_autentication(owner);
                self.state.debit(owner, amount).await;
                self.finish_transfer_to_account(amount, target_account)
                    .await;
            }
        }
    }

    //How do we handle incoming message coming from a different chain.
    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::Credit { amount, owner } => self.state.credit(owner, amount).await,
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl FungibleContract {
    fn check_account_autentication(&mut self, owner: Owner) {
        assert_eq!(
            self.runtime.authenticated_signer(),
            Some(owner),
            "Incorrect authentication"
        )
    }

    async fn finish_transfer_to_account(&mut self, amount: Amount, account: Account) {
        if account.chain_id == self.runtime.chain_id() {
            self.state.credit(account.owner, amount).await;
        } else {
            let message = Message::Credit {
                owner: account.owner,
                amount,
            };
            self.runtime
                .prepare_message(message)
                .with_authentication()
                .send_to(account.chain_id)
        }
    }
}
