use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Storage, DepsMut, Binary, StdResult, to_binary, StdError};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use sp_snip20::sp_snip20::Snip20;

use super::error::Snip20Error;


pub static CONFIG_KEY: &[u8] = b"snip_20_config";


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Snip20State {
    known_snip_20: Vec<Snip20>,
}

fn config(storage: &mut dyn Storage) -> Singleton<Snip20State> {
    singleton(storage, CONFIG_KEY)
}

fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Snip20State> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn initialize_snip_20_state(
    storage: &mut dyn Storage,
){
    let snip_20_state = Snip20State {
        known_snip_20: vec![],
    };
    config(storage).save(&snip_20_state).unwrap();
}

pub fn add_snip_20(
    deps: &mut DepsMut,
    snip20: Snip20
) -> Result<(), Snip20Error> {
    let mut conf = config(deps.storage);
    let mut state = conf.load()?;

    if !state.known_snip_20.iter().any(|s| s.get_contract().get_contract_info() == snip20.get_contract().get_contract_info()) {
        state.known_snip_20.push(snip20);
    }
    conf.save(&state)?;
    Ok(())
}


pub fn get_snip_20_contract(
    storage: &dyn Storage,
    index: usize,
) -> Result<Snip20, Snip20Error> {
    let state = config_read(storage).load()?;
    match state.known_snip_20.get(index) {
        Some(snip20_contract) => Ok(snip20_contract.clone()),
        None => Err(Snip20Error::InvalidIndex(index)),
    }
}

pub fn get_registered_snip_20s(storage: &dyn Storage) -> StdResult<Binary> {
    let state = config_read(storage).load()?;
    to_binary(&state.known_snip_20)
}

pub fn check_known_snip_20(
    storage: &dyn Storage,
    contract_addr: &String,
) -> Result<(), Snip20Error> {
    let state = config_read(storage).load()?;
    if !state
        .known_snip_20
        .iter()
        .any(|snip20| snip20.get_contract().get_address() == *contract_addr)
    {
        return Err(Snip20Error::UnknownSnip20(contract_addr.to_string()));
    }
    Ok(())
}

pub fn get_first_snip20(
    storage: &dyn Storage,
) -> StdResult<Snip20> {
    let state = config_read(storage).load()?;
    state.known_snip_20.get(0)
        .cloned()
        .ok_or_else(|| StdError::generic_err("No SNIP-20 token found"))
}

