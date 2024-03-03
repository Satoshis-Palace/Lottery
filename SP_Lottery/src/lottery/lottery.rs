use cosmwasm_std::{
    Addr, DepsMut, Env, Response, StdError, StdResult, Storage, Uint128, Uint64,
};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use secret_toolkit::storage::Keymap;
use serde::{Deserialize, Serialize};

use crate::{
    integrations::snip_20_manager::state::get_first_snip20,
    lottery::ticket::get_tickets_user_priv,
    state::State,
};

use super::{
    constants::{
        LOTTERY_CONFIG, LOTTERY_ID_CONFIG, STATE_KEY, TICKETS_BOUGHT_PER_ADDRESS_KEY,
        TOTAL_MONEY_COLLECTED_CONFIG,
    },
    error::LotteryError,
    random::Random,
    ticket::{Ticket, TicketLotteryKey, TICKETSUSERS},
};
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, JsonSchema)]
pub struct Lottery {
    pub difficulty_num: Uint64,
    pub cost: Uint128,
    pub start_time: Uint64,
    pub end_time: Uint64,
    pub numbers: Vec<Uint64>, // Changed to Vec<Uint64>
    pub is_redeemed: bool,
    pub amount: Uint128, // Amount of money for the lottery
    pub length: Uint64,
    pub(crate) tickets_sold: Uint64, // Length of the lottery
}

impl Default for Lottery {
    fn default() -> Self {
        Lottery {
            difficulty_num: Uint64::new(0),
            cost: Uint128::new(0),
            start_time: Uint64::new(0),
            end_time: Uint64::new(0),
            numbers: Vec::new(), // Initialize as an empty Vec
            is_redeemed: false,
            amount: Uint128::new(0),
            length: Uint64::new(0),
            tickets_sold: Uint64::new(0),
        }
    }
}

static LOTTERIES: Keymap<u64, Lottery> = Keymap::new(LOTTERY_CONFIG);
static TICKETS_BOUGHT_PER_ADDRESS: Keymap<Addr, u64> = Keymap::new(TICKETS_BOUGHT_PER_ADDRESS_KEY);
const LOTTERY_COUNT_KEY: &[u8] = LOTTERY_ID_CONFIG;
pub const TOTAL_MONEY_COLLECTED_KEY: &[u8] = TOTAL_MONEY_COLLECTED_CONFIG;
const SECONDS_PER_DAY: u64 = 86_400;

pub fn save_lottery(storage: &mut dyn Storage, id: u64, lottery: Lottery) -> StdResult<()> {
    let _ = LOTTERIES
        .insert(storage, &id, &lottery)
        .map_err(|_err| LotteryError::LotterySaveFail);
    Ok(())
}
pub fn get_lottery(storage: &dyn Storage, id: u64) -> Option<Lottery> {
    return LOTTERIES.get(storage, &id);
}
pub fn get_batch_lotteries(storage: &dyn Storage, start_id: u64, end_id: u64) -> StdResult<Vec<Lottery>> {
    let count = get_lottery_count(storage).load()?;

    // Check if the start and end IDs are within the valid range
    if start_id == 0 || start_id > count || end_id < start_id || end_id > count {
        return Err(StdError::generic_err("Requested lottery ID range is out of scope"));
    }

    let mut lotteries = Vec::new();

    // Adjusted range to be within scope
    for id in start_id..=end_id {
        if let Some(lottery) = get_lottery(storage, id) {
            lotteries.push(lottery);
        }
    }

    Ok(lotteries)
}

pub fn fund_and_start_lottery(storage: &mut dyn Storage, lottery: Lottery) -> StdResult<()> {
    // Create a block to limit the scope of the mutable borrow
    { //TODO get the lottery count a better way here or asumme 0
        let mut lottery_count = Singleton::new(storage, LOTTERY_COUNT_KEY);
        let count = lottery_count.load().unwrap_or(0); //If unset default to 0

        if count != 0 {
            return Err(StdError::generic_err(
                "The Lottery has already been started!",
            ));
        }

        // Increment and save the lottery count
        lottery_count.save(&1)?;
    }

    // Initialize and save the new lottery
    save_lottery(storage, 1, lottery)?;

    Ok(())
}

//TODO make thius only callable by the admin
pub fn update_lottery(
    storage: &mut dyn Storage,
    difficulty_num: Uint64,
    length: Uint64,
    cost: Uint128,
) -> StdResult<()> {
    // Load the current lottery count
    let count = get_lottery_count(storage).load()?;

    // The ID of the latest lottery is count
    let lottery_id = count;

    // Retrieve the existing lottery
    let mut lottery = LOTTERIES
        .get(storage, &lottery_id)
        .ok_or(StdError::generic_err("Latest lottery not found"))?;

    // Update the lottery fields
    lottery.difficulty_num = difficulty_num;
    lottery.length = length;
    lottery.cost = cost;

    // Save the updated lottery back to storage
    save_lottery(storage, lottery_id, lottery)
}

pub fn pull_lottery_numbers(
    env: Env,
    storage: &mut dyn Storage,
    skip_time_check: bool,
) -> StdResult<Response> {
    let count = get_lottery_count(storage).load()?;
    let current_lottery_id = count;
    let mut lottery = LOTTERIES
        .get(storage, &current_lottery_id)
        .ok_or_else(|| StdError::generic_err("Lottery not found"))?;

    // Convert lottery.length to u64 and calculate the expected time
    let length_in_seconds = lottery.length.u64();
    let expected_block_time = lottery.start_time.u64() + length_in_seconds;

    // Perform the time check only if skip_time_check is false
    if !skip_time_check && env.block.time.seconds() <= expected_block_time {
        return Err(StdError::generic_err(format!(
            "Conditions not met for pulling numbers. Current block: {}, expected block: {}",
            env.block.time.seconds(),
            expected_block_time
        )));
    }

    if !lottery.numbers.is_empty() {
        return Err(StdError::generic_err(format!(
            "Numbers have been pulled for lottery id {}",
            current_lottery_id
        )));
    }

    let mut random = Random::new(&env); //TODO make sure this uses secretVRF
    let difficulty_num = lottery.difficulty_num.u64() as usize;
    let mut numbers = Vec::new();
    for _ in 0..difficulty_num {
        let random_number = random.get_random_number_up_to(99) as u64;
        numbers.push(Uint64::new(random_number));
    }

    lottery.numbers = numbers;
    lottery.end_time = Uint64::new(env.block.time.seconds()); //TODO consider not having this here as its more of a pull numbers thing/ Logic was just not working great
    save_lottery(storage, current_lottery_id, lottery)?;

    Ok(Response::new()
        .add_attribute("action", "pull_lottery_numbers")
        .add_attribute("status", "Numbers pulled successfully"))
}

pub fn increment_lottery(env: Env, mut deps: &mut DepsMut) -> StdResult<()> {
    let mut last_lottery_amount = Uint128::new(0);
    let mut recent_lottery = None;

    // Fetch the necessary data first

    let count = get_lottery_count(deps.storage).load()?;

    //If there is more than 1 lottery already run this. if not just create an empty lottery
    if count >= 2 {
        //Collect two lotteries ago amount
        if let Some(last_lottery) = LOTTERIES.get(deps.storage, &(count - 1)) {
            last_lottery_amount = last_lottery.amount;
            // Prepare the two lotteries ago lottery for zeroing out
            recent_lottery = Some(last_lottery);
        }

        // Zero out the amount from the last lottery
        if let Some(mut lottery) = recent_lottery {
            lottery.amount = Uint128::new(0);
            save_lottery(deps.storage, count - 1, lottery)?;
        }
        //Collect current lottery and move the amount over
        if let Some(mut current_lottery) = LOTTERIES.get(deps.storage, &(count)) {
            current_lottery.amount += last_lottery_amount; //Add its currently collected to the past
            save_lottery(deps.storage, count, current_lottery)?;
        }
    }

    // Prepare the new lottery
    let mut new_lottery = LOTTERIES.get(deps.storage, &(count)).unwrap();

    //Make the block the current block, make it not redeemed and take the money from 2 lotteries ago and move it here. TODO consider calculating totaltickets bought at this point to save gas and deriving it on the current lottery 
    new_lottery.start_time = Uint64::new(env.block.time.seconds());
    new_lottery.end_time = Uint64::new(0);
    new_lottery.is_redeemed = false;
    new_lottery.amount = Uint128::new(0);
    new_lottery.numbers = Vec::new();
    new_lottery.tickets_sold = Uint64::new(0);

    // Save the new lottery and increment the count
    save_lottery(deps.storage, count + 1, new_lottery)?;
    {
        let _ = try_increment_lottery_count(&mut deps, env);

        // let mut lottery_count = Singleton::new(storage, LOTTERY_COUNT_KEY);
        // lottery_count.save(&(count + 1))?;
    }

    Ok(())
}

//Functions to deal with lottery count
pub fn set_lottery_count(storage: &mut dyn Storage) -> Singleton<u64> {
    singleton(storage, LOTTERY_COUNT_KEY)
}
pub fn get_lottery_count(storage: &dyn Storage) -> ReadonlySingleton<u64> {
    singleton_read(storage, LOTTERY_COUNT_KEY)
}
pub fn try_increment_lottery_count(deps: &mut DepsMut, _env: Env) -> StdResult<Response> {
    set_lottery_count(deps.storage).update(|mut state| -> Result<_, StdError> {
        state += 1;
        Ok(state)
    })?;

    deps.api.debug("count incremented successfully");
    Ok(Response::default())
}
//Functions to deal with lottery money collected
pub fn set_total_money_collected(storage: &mut dyn Storage) -> Singleton<Uint128> {
    singleton(storage, TOTAL_MONEY_COLLECTED_KEY)
}

pub fn get_total_money_collected(storage: &dyn Storage) -> ReadonlySingleton<Uint128> {
    singleton_read(storage, TOTAL_MONEY_COLLECTED_KEY)
}
pub fn update_total_money_collected(storage: &mut dyn Storage, amount: Uint128) -> StdResult<()> {
    //TODO remove the stdresult and have it return nothing??
    let mut total_money_collected = singleton::<Uint128>(storage, TOTAL_MONEY_COLLECTED_KEY);

    // Retrieve current total, or use zero if not set
    let current_total = total_money_collected.load().unwrap_or(Uint128::zero());

    // Update total money collected
    total_money_collected.save(&(current_total + amount))?;

    Ok(())
}

//Functions to deal with internal ticket purchases TODO rework with new multi ticket crap
pub fn get_tickets_for_address(storage: &dyn Storage, address: &Addr) -> StdResult<u64> {
    Ok(TICKETS_BOUGHT_PER_ADDRESS
        .get(storage, address)
        .unwrap_or_default())
}
pub fn increment_tickets_for_address(
    storage: &mut dyn Storage,
    address: &Addr,
    amount_to_add: u64,
) -> StdResult<()> {
    let current_count = TICKETS_BOUGHT_PER_ADDRESS
        .get(storage, address)
        .unwrap_or_default();

    TICKETS_BOUGHT_PER_ADDRESS.insert(storage, address, &(current_count + amount_to_add))?;

    Ok(())
}
//Functions for altering the state
pub fn state_singleton(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, STATE_KEY)
}
pub fn state_singleton_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, STATE_KEY)
}

pub fn query_owner_address(storage: &dyn Storage) -> StdResult<String> {
    let state = state_singleton_read(storage).load()?;
    Ok(state.owner.to_string())
}
// Helper function to check if a ticket's numbers match the lottery's winning numbers
pub fn is_winning_ticket(
    storage: &dyn Storage,
    ticket: &Ticket,
    lottery_id: u64,
) -> StdResult<bool> {
    // Get the lottery by provided id
    let lottery = get_lottery(storage, lottery_id).ok_or_else(|| {
        StdError::generic_err(format!("Lottery with id {} not found", lottery_id))
    })?;

    // Check if the ticket's number vector is empty
    if ticket.numbers.is_empty() {
        return Err(StdError::generic_err(
            "Ticket numbers have not been pulled yet",
        ));
    }

    // Get difficulty number and winning numbers from the specified lottery
    let difficulty_num = lottery.difficulty_num.u64() as usize;
    let winning_numbers = &lottery.numbers;

    // Ensure there are enough numbers in both the ticket and the winning numbers
    if ticket.numbers.len() != difficulty_num {
        return Err(StdError::generic_err(format!(
            "Wrong numbers for comparison. Expected {}. Actual {}. This shouldn't happen",
            difficulty_num,
            ticket.numbers.len()
        )));
    }
    let tickets_user = get_tickets_user_priv(storage, &ticket, lottery_id)?;
    println!(
        "user ticket: {:?}, winning ticket: {:?}. this one frfr {:?}",
        ticket.numbers, winning_numbers, tickets_user
    );
    // Compare the first 'difficulty_num' numbers in both vectors
    Ok(ticket
        .numbers
        .iter()
        .zip(winning_numbers.iter())
        .take(difficulty_num)
        .all(|(&ticket_num, &winning_num)| ticket_num == winning_num))
}
pub fn redeem_ticket(
    deps: &mut DepsMut,
    user_address: Addr,
    ticket: Ticket,
    lottery_id: u64, //TODO consider removing this as it can be infered as it only works for the current lottery -1
) -> StdResult<Response> {
    // Get the current lottery count and use the last lottery id for all logic
    let current_lottery_count = get_lottery_count(deps.storage).load()?;

    // // Ensure the lottery_id refers to the most recent past lottery TODO this may be logiced out already consider removing
    if lottery_id != current_lottery_count - 1 {
        return Err(StdError::generic_err(
            "Can only redeem tickets from the most recent past lottery",
        ));
    }
    // Check if the ticket is a winner
    let is_winner = is_winning_ticket(deps.storage, &ticket, lottery_id).map_err(|e| {
        StdError::generic_err(format!("Failed to check if ticket is a winner: {}", e))
    })?;

    if !is_winner {
        return Err(StdError::generic_err("Ticket is not a winner"));
    }

    // Get the lottery by provided id
    let mut lottery = get_lottery(deps.storage, lottery_id)
        .ok_or_else(|| StdError::generic_err("Lottery not found"))?;

    // Define a placeholder address (e.g., contract's own address)
    let placeholder_address = deps.api.addr_validate("secretfakeaddress")?;

    // Update the TICKETSUSERS mapping: replace the user's address with the placeholder address
    let key = TicketLotteryKey {
        ticket: ticket.clone(),
        lottery_id,
    };
    // Retrieve the vector of addresses for the given ticket
    let mut ticket_owner_addresses = TICKETSUSERS.get(deps.storage, &key).unwrap_or_default();
    println!(
        "ticket_owner_addresses: {:?}, user_address: {:?}",
        ticket_owner_addresses, user_address
    );

    // Check if the redeeming user is one of the valid addresses
    if !ticket_owner_addresses.contains(&user_address) {
        return Err(StdError::generic_err("User is not a valid ticket holder"));
    }

    let mut found = false;
    let mut index = 0;
    //Search through all of the winnign addresses and see if the users address is a winner
    for (i, addr) in ticket_owner_addresses.iter().enumerate() {
        if *addr == user_address {
            println!("*addr: {:?}", *addr);
            found = true;
            index = i;
            break;
        }
    }
    if found {
        // Replace the user's address with a placeholder to indicate redemption
        ticket_owner_addresses[index] = placeholder_address.clone();

        // Update the TICKETSUSERS map
        TICKETSUSERS
            .insert(deps.storage, &key, &ticket_owner_addresses)
            .map_err(|_| StdError::generic_err("Failed to update ticket owners"))?;
    } else {
        // If the user's address is not found or has already been replaced
        return Err(StdError::generic_err(
            "Ticket not found or already redeemed",
        ));
    }

    // Count the number of valid (non-placeholder) addresses
    let valid_address_count = ticket_owner_addresses.len();
    println!("ticket_owner_addresses: {:?}", ticket_owner_addresses);
    if valid_address_count == 0 {
        return Err(StdError::generic_err("No valid tickets found"));
    }
    // Calculate the winning amount for each valid address
    let individual_winning_amount = lottery.amount.u128() / valid_address_count as u128;
    let winning_amount = Uint128::from(individual_winning_amount);

    // Mark the current lottery as redeemed and zero out its amount
    //lottery.is_redeemed = true; //TODO remove this feature
    lottery.amount = lottery.amount - winning_amount;
    save_lottery(deps.storage, current_lottery_count, lottery)?;

    // Logic to transfer the winning amount to the user
    let first_snip20 = get_first_snip20(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "redeem_ticket")
        .add_attribute("lottery_id", current_lottery_count.to_string())
        .add_attribute("user", user_address.clone())
        .add_attribute("message", format!("You won {} amount", winning_amount))
        .add_message(first_snip20.create_send_msg(user_address.to_string(), winning_amount)?))
}
