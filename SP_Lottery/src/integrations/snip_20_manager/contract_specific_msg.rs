use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::lottery::ticket::Ticket;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContractMessage {
    Snip20BlackJack(Snip20LotteryMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Snip20LotteryMsg {
    BuyTicketMulti {
        tickets: Vec<Ticket>,
        sender: Option<Addr>,
    },
}
