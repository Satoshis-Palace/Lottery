use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use crate::error::ContractError;
use crate::integrations::snip_20_manager::actions::{try_register, try_receive, try_redeem};
use crate::integrations::snip_20_manager::state::{initialize_snip_20_state, get_snip_20_contract};
use crate::lottery::actions::{ try_fund_and_start_lottery, try_pull_numbers_and_increment, try_pull_numbers_and_increment_admin, try_redeem_ticket, try_update_admin};
use crate::lottery::lottery::{get_batch_lotteries, get_lottery, get_lottery_count, get_tickets_for_address, get_total_money_collected, query_owner_address, state_singleton};
use crate::lottery::ticket::{get_batch_tickets_user, get_tickets_user_pub, UserLotteryKey, USERSTICKETS};
use crate::msg::{ExecuteMsg, InstantiateMsg, LatestLotteryResponse, OwnerAddresResponse, QueryMsg, Snip20AddressResponse, TicketPriceResponse};
use crate::state::{config, State};
use crate::viewingkeys::viewing_keys::{try_set_key, try_create_key, validate_query};

#[entry_point]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State { owner: info.sender.clone() };
    state_singleton(deps.storage).save(&state)?;
    let _ = try_fund_and_start_lottery(&mut deps, _env, msg.config.amount, msg.config.cost, msg.config.length, msg.config.difficulty);
    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());
    config(deps.storage).save(&state)?;
    initialize_snip_20_state(deps.storage);
    Ok(Response::default())
}

#[entry_point]
pub fn execute(mut deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        // SNIP-20 Msgs
        ExecuteMsg::Register { snip_20, entropy } => try_register(&mut deps, env.clone(),info, snip_20, &entropy),
        ExecuteMsg::Receive { sender, from, amount, memo, msg } => try_receive(&mut deps, env.clone(), info.clone(), sender, from, amount, memo, msg),
        ExecuteMsg::Redeem { addr, hash, to, amount, denom } => try_redeem(&mut deps, addr, hash, to, amount, denom), //Consider removing
        //

        //NOT SNIP20
        ExecuteMsg::RedeemTicket { ticket } => try_redeem_ticket(deps, info.sender,  ticket),
        ExecuteMsg::PullLotteryNumbers {} => try_pull_numbers_and_increment(&mut deps, env),
        ExecuteMsg::PullLotteryNumbersAdmin {difficulty_num, length, cost} => try_pull_numbers_and_increment_admin(&mut deps, env,difficulty_num, length, cost, info.sender),
        ExecuteMsg::UpdateAdmin {address} => try_update_admin(&mut deps, info.sender, address),

        //Viewing Keys TODO look to see if "entropy" is named properly
        ExecuteMsg::CreateViewingKey { entropy, .. } => try_create_key(deps, env, info, entropy),
        ExecuteMsg::SetViewingKey { key, .. } => try_set_key(deps, info, key),
}
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTicketsUser { ticket, lottery_id } => {
            to_binary(&get_tickets_user_pub(deps.storage, &ticket, lottery_id).unwrap_or_default())
        },
        QueryMsg::BatchGetTicketsUser { tickets, lottery_id } => {
            to_binary(&get_batch_tickets_user(deps.storage, &tickets, lottery_id).unwrap_or_default())
        },
        QueryMsg::GetLottery { id } => {
            to_binary(&get_lottery(deps.storage, id).unwrap_or_default())
        },
        QueryMsg::GetLatestLottery { } => {
            let data = get_lottery_count(deps.storage).load()?;
            let response = LatestLotteryResponse{id : data};
            to_binary( &response)
        },
        QueryMsg::GetBatchLottery { start_id, end_id } => {
            let lotteries = get_batch_lotteries(deps.storage, start_id, end_id)?;
            to_binary(&lotteries)
        },
        QueryMsg::GetTicketPrice { id } => {
            let data = get_lottery(deps.storage, id).unwrap_or_default().cost;
            let response = TicketPriceResponse{cost: data};
            to_binary( &response)
        },
        QueryMsg::GetSnip { id } => {
            let data = get_snip_20_contract(deps.storage, id.try_into().unwrap()).unwrap();
            let response = Snip20AddressResponse{address: data};
            to_binary( &response)
        },
        QueryMsg::GetOwner {  } => {
            let data = query_owner_address(deps.storage).unwrap();
            let response = OwnerAddresResponse{address: data};
            to_binary( &response)
        },
        QueryMsg::GetTotalMoneyCollected {  } => {
            let data = get_total_money_collected(deps.storage).load()?;
            to_binary( &data)
        },
        //TODO make the secrecy better
        _ => viewing_keys_queries(deps, msg),
    }
}
    pub fn viewing_keys_queries(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
        validate_query(&deps, &msg)?;
    
        return match msg {
            QueryMsg::GetUsersTickets { address, lottery_id, key:_ } => {
                let lottery_key = UserLotteryKey { address, lottery_id };
                to_binary(&USERSTICKETS.get(deps.storage, &lottery_key).unwrap_or_default())
            },
            QueryMsg::GetUserTotalTickets { address, key:_ } => {
                to_binary(&get_tickets_for_address(deps.storage, &address).unwrap_or_default())
            },
            _ => Err(ContractError::QueryDoesNotRequireAuthentication.into()),
        };
    }

    