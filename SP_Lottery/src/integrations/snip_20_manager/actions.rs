use cosmwasm_std::{
    Addr, Binary, ContractInfo, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use secret_toolkit::snip20::{register_receive_msg, redeem_msg};
use sp_snip20::sp_snip20::Snip20;

use super::{
    constants::BLOCK_SIZE,
    contract_specific_handler::contract_try_receive,
    state::{add_snip_20, check_known_snip_20},
};

pub fn try_register(
    mut deps: &mut DepsMut,
    env: Env,
    info: MessageInfo,
    snip_20_contract_info: ContractInfo,
    entropy: &[u8],
) -> StdResult<Response> {
    let snip20 = Snip20::new(
        &mut deps,
        &env,
        &info,
        &snip_20_contract_info,
        entropy,
    )?;
    
    let set_view_key_msg = snip20.create_set_view_key_msg()?;
    
    add_snip_20(&mut deps, snip20)?;

    // let msg = to_binary(&Snip20Msg::register_receive(env.contract.code_hash))?;
    let register_receive_msg = register_receive_msg(
        env.contract.code_hash,
        None,
        BLOCK_SIZE,
        snip_20_contract_info.code_hash,
        snip_20_contract_info.address.to_string(),
    )?;

    Ok(Response::new()
        .add_message(register_receive_msg)
        .add_message(set_view_key_msg)
    )

}

#[allow(unused_variables)]
pub fn try_receive(
    deps: &mut DepsMut,
    env: Env,
    info: MessageInfo,
    sender: Addr,
    from: Addr,
    amount: Uint128,
    memo: Option<String>,
    msg: Binary,
) -> StdResult<Response> {
    check_known_snip_20(deps.storage, &info.clone().sender.to_string())?;

    contract_try_receive(deps, env, info, sender, from, amount, memo, msg)
}

#[allow(unused_variables)]
pub fn try_redeem(
    deps: &mut DepsMut,
    addr: String,
    hash: String,
    to: Addr,
    amount: Uint128,
    denom: Option<String>,
) -> StdResult<Response> {
    check_known_snip_20(deps.storage, &addr)?;

    let unwrapped_denom = denom.unwrap_or("uscrt".to_string());

    let secret_redeem = &redeem_msg(
        amount,
        Some(unwrapped_denom.clone()),
        None,
        BLOCK_SIZE,
        hash,
        addr,
    )?;

    Ok(Response::new().add_message(secret_redeem.to_owned()))
}