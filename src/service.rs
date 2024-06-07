#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::{Arc, Mutex};

use self::state::Fungible;
use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    base::{Amount, Owner, WithServiceAbi},
    graphql::GraphQLMutationRoot,
    views::{MapView, View, ViewStorageContext},
    Service, ServiceRuntime,
};
use my_fungible::Operation;

#[derive(Clone)]
pub struct FungibleService {
    state: Arc<Fungible>,
    runtime: Arc<Mutex<ServiceRuntime<Self>>>,
}

linera_sdk::service!(FungibleService);

impl WithServiceAbi for FungibleService {
    type Abi = my_fungible::ApplicationAbi;
}

impl Service for FungibleService {
    type Parameters = ();

    // Instanceous a fungible service by loading the state from storage
    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Fungible::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        FungibleService {
            state: Arc::new(state),
            runtime: Arc::new(Mutex::new(runtime)),
        }
    }

    //Query coming from outside world how do we reason about giving back a response.
    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema =
            Schema::build(self.clone(), Operation::mutation_root(), EmptySubscription).finish();
        schema.execute(query).await
    }
}

#[Object]
impl FungibleService {
    async fn accounts(&self) -> &MapView<Owner, Amount> {
        &self.state.accounts
    }
}
