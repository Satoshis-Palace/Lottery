use cosmwasm_std::{DepsMut, Env, MessageInfo, Addr, Uint128, Binary, Response, StdResult, from_binary};

use crate::lottery::actions::try_buy_ticket_multi;

use super::contract_specific_msg::Snip20LotteryMsg;


#[allow(unused_assignments)]
#[allow(unused_variables)]
pub fn contract_try_receive(
    mut deps: &mut DepsMut,
    env: Env,
    info: MessageInfo,
    sender: Addr,
    _from: Addr,
    sent_amount: Uint128,
    memo: Option<String>,
    msg: Binary,
) -> StdResult<Response> {
    let msg: Snip20LotteryMsg = from_binary(&msg)?;
	
    match msg {
        Snip20LotteryMsg::BuyTicketMulti {tickets,sender } =>{
			return try_buy_ticket_multi(&mut deps, sender, sent_amount, tickets)
		}
    };
}