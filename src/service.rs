#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Fungible;
use linera_sdk::{
    base::WithServiceAbi,
    views::{View, ViewStorageContext},
    Service, ServiceRuntime,
};

pub struct FungibleService {
    state: Fungible,
    runtime: ServiceRuntime<Self>,
}

linera_sdk::service!(FungibleService);

impl WithServiceAbi for FungibleService {
    type Abi = my_fungible::ApplicationAbi;
}

impl Service for FungibleService {
    type Parameters = ();

    // Instanceous a fungible service by loading the state from Storage
    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Fungible::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        FungibleService { state, runtime }
    }

    //Query coming from outside world how do we reason about giving back a response.
    async fn handle_query(&self, _query: Self::Query) -> Self::QueryResponse {
        panic!("Queries not supported by application");
    }
}
