use linera_sdk::base::{Amount, ChainId, ContractAbi, Owner, ServiceAbi};
use linera_sdk::graphql::GraphQLMutationRoot;
use linera_sdk::views::linera_views::serde::{Deserialize, Serialize};

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = ();
    type Response = ();
}

impl ServiceAbi for ApplicationAbi {
    type Query = ();
    type QueryResponse = ();
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Transfer {
        owner: Owner,
        amount: Amount,
        target_account: Account,
    },
}

pub struct Account {
    pub chain_id: ChainId,
    pub owner: Owner,
}
