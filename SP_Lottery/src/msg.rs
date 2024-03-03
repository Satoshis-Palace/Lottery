use cosmwasm_std::{Addr, Binary, ContractInfo, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_snip20::sp_snip20::Snip20;

use crate::lottery::ticket::Ticket;
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub amount: Uint128,
    pub cost: Uint128,
    pub length: Uint64,
    pub difficulty: Uint64,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RedeemTicket {
        ticket: Ticket,
    },
    PullLotteryNumbers {},
    PullLotteryNumbersAdmin {
        difficulty_num: Uint64,
        length: Uint64,
        cost: Uint128,
    },
    UpdateAdmin {
        address: Addr,
    },

    //SNIP 20
    Register {
        snip_20: ContractInfo,
        entropy: Binary,
    },
    Receive {
        sender: Addr,
        from: Addr,
        amount: Uint128,
        memo: Option<String>,
        msg: Binary,
    },
    Redeem {
        addr: String,
        hash: String,
        to: Addr,
        amount: Uint128,
        denom: Option<String>,
    },
    // Viewing Keys
    CreateViewingKey {
        entropy: String,
        padding: Option<String>,
    },
    SetViewingKey {
        key: String,
        padding: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetUsersTickets { address: Addr, lottery_id: u64, key: String },
    GetUserTotalTickets { address: Addr, key: String },
    GetTicketsUser { ticket: Ticket, lottery_id: u64 },
    BatchGetTicketsUser {tickets: Vec<Ticket>, lottery_id: u64 },
    GetLottery { id: u64 },
    GetLatestLottery { },
    GetBatchLottery { start_id: u64, end_id: u64 },
    GetTicketPrice { id: u64 },
    GetSnip { id: u64},
    GetOwner { },
    GetTotalMoneyCollected { }
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LatestLotteryResponse{
 pub id : u64
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TicketPriceResponse{
    pub cost : Uint128
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Snip20AddressResponse{
    pub address : Snip20
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OwnerAddresResponse{
    pub address : String
}
// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

