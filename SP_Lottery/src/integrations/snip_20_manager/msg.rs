use cosmwasm_std::{Addr, Uint128, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Snip20Msg {
    Register {
        reg_addr: Addr,
        reg_hash: String,
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
}