use std::fmt;

use cosmwasm_std::{Uint64, Addr, Storage, StdResult};
use schemars::JsonSchema;
use secret_toolkit::storage::Keymap;
use serde::{Deserialize, Serialize};

use super::{constants::{USERS_TICKETS_CONFIG, TICKETS_USERS_CONFIG}, error::LotteryError};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Ticket{
    pub numbers: Vec<Uint64>
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserLotteryKey {
    pub address: Addr,
    pub lottery_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct TicketLotteryKey {
    pub ticket: Ticket,
    pub lottery_id: u64,
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Join the numbers with a comma and a space, and write to the formatter
        let numbers_str = self.numbers
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "Ticket Numbers: [{}]", numbers_str)
    }
}
pub static USERSTICKETS: Keymap<UserLotteryKey, Vec<Ticket>> = Keymap::new(USERS_TICKETS_CONFIG);
pub static TICKETSUSERS: Keymap<TicketLotteryKey, Vec<Addr>> = Keymap::new(TICKETS_USERS_CONFIG);

pub fn save_users_tickets(storage: &mut dyn Storage, address: Addr, lottery_id: u64, tickets: Vec<Ticket>) -> StdResult<()> {
    let key = UserLotteryKey { address, lottery_id };
    let _ = USERSTICKETS.insert(storage, &key, &tickets).map_err(|_err| LotteryError::TicketSaveFail);
    Ok(())
}

pub fn save_tickets_user(storage: &mut dyn Storage, ticket: Ticket, lottery_id: u64, addresses: Vec<Addr>) -> StdResult<()> {
    let key = TicketLotteryKey { ticket: ticket.clone(), lottery_id };
    let _ = TICKETSUSERS.insert(storage, &key, &addresses).map_err(|_err| LotteryError::TicketSaveFail);
    Ok(())
}

pub fn get_users_tickets(storage: &dyn Storage, address: Addr, lottery_id: u64) -> StdResult<Vec<Ticket>> {
    let key = UserLotteryKey { address, lottery_id };
    Ok(USERSTICKETS.get(storage, &key).unwrap_or_default())
}

pub fn get_tickets_user_pub(storage: &dyn Storage, ticket: &Ticket, lottery_id: u64) -> StdResult<usize> {
    let key = TicketLotteryKey { ticket: ticket.clone(), lottery_id };

    // Retrieve the vector of addresses for the given ticket.
    let addresses = match TICKETSUSERS.get(storage, &key) {
        Some(address_vec) => address_vec,
        None => Vec::new(), // If there's no vector, return an empty one
    };

    // Return the length of the vector.
    Ok(addresses.len())
}
pub fn get_tickets_user_priv(storage: &dyn Storage, ticket: &Ticket, lottery_id: u64) -> StdResult<Vec<Addr>> {
    let key = TicketLotteryKey { ticket: ticket.clone(), lottery_id };
    Ok(TICKETSUSERS.get(storage, &key).unwrap_or_default())
}

pub fn get_batch_tickets_user(storage: &dyn Storage, tickets: &[Ticket], lottery_id: u64) -> StdResult<Vec<Ticket>> {
    let mut found_tickets = Vec::new();

    for ticket in tickets {
        let key = TicketLotteryKey { ticket: ticket.clone(), lottery_id };

        // Check if the ticket exists
        if TICKETSUSERS.get(storage, &key).is_some() {
            // If the ticket exists, add its numbers to the list
            found_tickets.push(ticket.clone());
        }
    }
    Ok(found_tickets)
}

