use async_graphql::{InputObject, Request, Response};
use linera_sdk::base::{Amount, ChainId, ContractAbi, Owner, ServiceAbi};
use linera_sdk::graphql::GraphQLMutationRoot;
use serde::{Deserialize, Serialize};

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for ApplicationAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Transfer {
        owner: Owner,
        amount: Amount,
        target_account: Account,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Credit { owner: Owner, amount: Amount },
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd, InputObject,
)]
pub struct Account {
    pub chain_id: ChainId,
    pub owner: Owner,
}
