use cosmwasm_std::{
    Addr, DepsMut, Env, Response, StdError, StdResult, Uint128, Uint64,
};

use crate::error::ContractError;

use super::{
    error::LotteryError,
    lottery::{
        fund_and_start_lottery, get_lottery, get_lottery_count, increment_lottery, increment_tickets_for_address, is_winning_ticket, pull_lottery_numbers, query_owner_address, redeem_ticket, save_lottery, state_singleton, update_lottery, update_total_money_collected, Lottery
    },
    ticket::{
         get_tickets_user_priv, get_users_tickets, save_tickets_user, save_users_tickets, Ticket, TicketLotteryKey, UserLotteryKey, TICKETSUSERS
    },
};

pub fn validate_ticket(deps: &mut DepsMut, ticket: &Ticket, user_address: Addr) -> StdResult<()> {
    let count = get_lottery_count(deps.storage).load()?;
    let current_lottery_id = if count > 0 {
        count
    } else {
        return Err(StdError::generic_err("No current lottery available"));
    };

    let current_lottery = get_lottery(deps.storage, current_lottery_id)
        .ok_or(StdError::generic_err("Current lottery not found"))?;

    // Check if the ticket is already purchased
    let key = TicketLotteryKey { ticket: ticket.clone(), lottery_id: current_lottery_id };
    let ticket_owner_addresses = TICKETSUSERS.get(deps.storage, &key).unwrap_or_default();

    if ticket_owner_addresses.contains(&user_address) {
        return Err(LotteryError::TicketAlreadyPurchased(ticket.clone()).into());
    }

    let expected_num_count = current_lottery.difficulty_num.u64() as usize;
    let actual_num_count = ticket.numbers.len();

    if actual_num_count > expected_num_count {
        return Err(StdError::generic_err(format!(
            "Too many numbers on your lottery ticket: provided {}, but expected {}",
            actual_num_count, expected_num_count
        )));
    }

    if actual_num_count < expected_num_count {
        return Err(StdError::generic_err(format!(
            "Too few numbers on your lottery ticket: provided {}, but expected {}",
            actual_num_count, expected_num_count
        )));
    }

    for &number in &ticket.numbers {
        if number.u64() >= 100 {
            return Err(StdError::generic_err(
                "Ticket numbers must be between 0 and 99",
            ));
        }
    }

    Ok(())
}

fn buy_ticket(
    deps: &mut DepsMut,
    address: Option<Addr>,
    ticket: Ticket,
) -> StdResult<Response> {
    let user_addr = address.ok_or(StdError::generic_err("Address is required"))?;

    // Validate the ticket
    validate_ticket(deps, &ticket, user_addr.clone())?;

    // Get the current lottery count and find the ID of the current lottery
    let count = get_lottery_count(deps.storage).load()?;
    let current_lottery_id = if count > 0 { count } else { 0 };

    // Create keys for USERSTICKETS and TICKETSUSERS
    let _user_lottery_key = UserLotteryKey {
        address: user_addr.clone(),
        lottery_id: current_lottery_id,
    };

    let _ticket_lottery_key = TicketLotteryKey {
        ticket: ticket.clone(),
        lottery_id: current_lottery_id,
    };

    // Fetch and update user's tickets
    let mut user_tickets = get_users_tickets(deps.storage, user_addr.clone(), current_lottery_id)?;
    user_tickets.push(ticket.clone());
    save_users_tickets(
        deps.storage,
        user_addr.clone(),
        current_lottery_id,
        user_tickets,
    )?;

    // Save the ticket-user mapping
    let mut tickets_user = get_tickets_user_priv(deps.storage, &ticket, current_lottery_id)?;
    //println!("user address: {:?}, user_address array: {:?}", user_addr.clone(), tickets_user.get(0).unwrap());
    tickets_user.push(user_addr.clone());
    save_tickets_user(
        deps.storage, 
        ticket.clone(), 
        current_lottery_id, 
        tickets_user.clone()
    )?;
    let tickets_user = get_tickets_user_priv(deps.storage, &ticket, current_lottery_id)?;
    println!("user address: {:?}, user_address array: {:?}", user_addr.clone(), tickets_user);

    let response = Response::new()
        .add_attribute("action", "buy_ticket")
        .add_attribute("buyer", user_addr.to_string())
        .add_attribute("ticket_numbers", format!("{:?}", ticket.numbers));

    Ok(response)
}

pub fn try_buy_ticket_multi( //TODO make this not call the buy ticket and instead do similar logic to avoid setting the increment variable 100 times
    deps: &mut DepsMut,
    address: Option<Addr>,
    amount: Uint128,
    tickets: Vec<Ticket>,
) -> StdResult<Response> {
    let user_addr = address.ok_or(StdError::generic_err("Address is required"))?;
    // Get the current lottery count and find the ID of the current lottery
    let count = get_lottery_count(deps.storage).load()?;
    let current_lottery_id = if count > 0 { count } else { 0 };
    // Retrieve the current lottery cost
    let current_lottery = get_lottery(deps.storage, current_lottery_id).unwrap();
    let cost_per_ticket = current_lottery.cost;

    // Convert the number of tickets to Uint128
    let num_tickets = Uint128::from(tickets.len() as u128);

    // Calculate total cost
    let total_cost = cost_per_ticket * num_tickets;

    // Now `total_cost` is a Uint128 and can be used in comparisons

    // Check if the provided amount is sufficient

        if amount.u128() < total_cost.into() {
            return Err(StdError::generic_err(
                "Insufficient funds for buying tickets",
            ));
    }
        if amount.u128() > total_cost.into() {
            return Err(StdError::generic_err(
                "Overly sufficient funds for buying tickets",
            ));
        }

    // Iterate over each ticket and try to buy it
    for ticket in tickets.clone() {
        buy_ticket(
            deps,
            Some(user_addr.clone()),
            ticket,
        )?;
    }
    //Add ticket cost amount to the current lottery's amount
    // Assuming amount is of type Option<Uint128>
    let mut current_lottery = get_lottery(deps.storage, current_lottery_id).unwrap();

    // Add ticket cost amount to the current lottery's amount
    current_lottery.amount += amount;
    current_lottery.tickets_sold += Uint64::from(tickets.len() as u64);
    // Save the updated lottery
    save_lottery(deps.storage, current_lottery_id, current_lottery)?;
    // Increase the ticket count for the provided address
    increment_tickets_for_address(deps.storage, &user_addr, tickets.len().try_into().unwrap())?;
    // Increase the total amount collected
    let _ = update_total_money_collected(deps.storage, amount);

    // Return success response
    Ok(Response::new().add_attribute("action", "buy_multiple_tickets"))
}

pub fn try_redeem_ticket(
    mut deps: DepsMut,
    user_address: Addr,
    ticket: Ticket,
) -> StdResult<Response> {
    // Get the current lottery count to determine the most recent past lottery ID
    let current_lottery_count = get_lottery_count(deps.storage).load()?;
    let lottery_id = current_lottery_count - 1;

    // Check if User owns ticket
    let key = TicketLotteryKey {
        ticket: ticket.clone(),
        lottery_id,
    };

    // Retrieve the vector of addresses for the given ticket
    let ticket_owner_addresses = TICKETSUSERS.get(deps.storage, &key).unwrap_or_default();

    // Check if the user's address is in the vector of ticket owner addresses
    if !ticket_owner_addresses.contains(&user_address) {
        return Err(LotteryError::TicketNotOwnedByUser.into());
    }

    // Check if the ticket is a winner
    let is_winner = is_winning_ticket(deps.storage, &ticket, lottery_id)?;
    if is_winner {
        // Redeem the ticket
        redeem_ticket(&mut deps, user_address, ticket, lottery_id)
    } else {
        // Ticket is not a winner
        Err(StdError::generic_err("Ticket is not a winning ticket"))
    }
}


pub fn try_fund_and_start_lottery(
    //TODO make thi only callable by the admin
    deps: &mut DepsMut,
    env: Env,
    amount: Uint128,
    cost: Uint128,
    length: Uint64,
    difficulty: Uint64,
) -> StdResult<Response> {
    // Prepare the new lottery object
    let lottery = Lottery {
        // Initialize the fields of the Lottery struct
        difficulty_num: difficulty,
        cost: cost,
        start_time: Uint64::new(env.block.time.seconds()),
        end_time: Uint64::new(0),
        numbers: Vec::new(),
        is_redeemed: false,
        amount: amount,
        length: length,
        tickets_sold: Uint64::new(0),
    };

    // Try to fund and start the lottery
    match fund_and_start_lottery(deps.storage, lottery) {
        Ok(_) => Ok(Response::new()
            .add_attribute("action", "fund_and_start_lottery")
            .add_attribute("status", "Lottery started successfully")),
        Err(e) => Err(StdError::generic_err(format!(
            "Failed to start lottery: {}",
            e
        ))),
    }
}

pub fn try_pull_numbers_and_increment(deps: &mut DepsMut, env: Env) -> StdResult<Response> {
    // First, try to pull lottery numbers
    pull_lottery_numbers(env.clone(), deps.storage, false)?;

    // Then, increment the lottery
    increment_lottery(env, deps)?;

    Ok(Response::new()
        .add_attribute("action", "pull_numbers_and_increment")
        .add_attribute(
            "status",
            "Lottery numbers pulled and incremented successfully",
        ))
}

pub fn try_pull_numbers_and_increment_admin(
    deps: &mut DepsMut,
    env: Env,
    difficulty_num: Uint64,
    length: Uint64,
    cost: Uint128,
    sender: Addr,
) -> StdResult<Response> {
    // Check if sender has permission
    let owner_address = query_owner_address(deps.storage)?;
    if sender != owner_address {
        return Err(ContractError::AddressIsNotOwner.into());
    }

    // First, try to pull lottery numbers
    pull_lottery_numbers(env.clone(), deps.storage, true)?;

    // Then, increment the lottery
    increment_lottery(env, deps)?;
    let _response = update_lottery(deps.storage, difficulty_num, length, cost);
    Ok(Response::new()
        .add_attribute("action", "pull_numbers_and_increment")
        .add_attribute(
            "status",
            "Lottery numbers pulled and incremented successfully",
        ))
}

pub fn try_update_admin(
    deps: &mut DepsMut,
    sender: Addr,
    new_admin: Addr,
) -> StdResult<Response> {

    state_singleton(deps.storage).update(|mut state| -> StdResult<_> {
        if sender != state.owner {
            return Err(ContractError::AddressIsNotOwner.into());
        }
        state.owner = new_admin;
        Ok(state)
    })?;
    
    Ok(Response::new()
        .add_attribute("action", "update_admin")
        .add_attribute("status", "Admin updated successfully"))
}
