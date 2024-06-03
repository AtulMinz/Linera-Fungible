use linera_sdk::base::{Amount, Owner};
use linera_sdk::views::{linera_views, MapView, RootView, ViewStorageContext};

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Fungible {
    pub accounts: MapView<Owner, Amount>,
}

//Implement logic as wrapper aroung the state.

//1. Initialise
//2. Balance
//3. Credit
//4. Debit
impl Fungible {
    pub async fn initialize_accounts(&mut self, account: Owner, amount: Amount) {
        self.accounts
            .insert(&account, amount)
            .expect("Error Initialising Accounts")
    }

    pub async fn balance(&self, account: &Owner) -> Amount {
        self.accounts
            .get(account)
            .await
            .expect("Failed to get balance")
            .unwrap_or(Amount::ZERO)
    }

    pub async fn credit(&mut self, account: Owner, amount: Amount) {
        let mut balance = self.balance(&account).await;
        balance.saturating_add_assign(amount);
        self.accounts
            .insert(&account, balance)
            .expect("Failed to credit")
    }

    pub async fn debit(&mut self, account: Owner, amount: Amount) {
        let mut balance = self.balance(&account).await;
        balance
            .try_sub_assign(amount)
            .expect("Insufficient balance for transfer");
        self.accounts
            .insert(&account, balance)
            .expect("Failed to update balance")
    }
}
