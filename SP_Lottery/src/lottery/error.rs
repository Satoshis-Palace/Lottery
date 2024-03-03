use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

use super::ticket::Ticket;

#[derive(Error, Debug)]
pub enum LotteryError {
    #[error("The Lottery failed to save")]
    LotterySaveFail,
    #[error("The Ticket failed to save")]
    TicketSaveFail,
    #[error("Lottery not found for this ID")]
    LotteryNotFound,
    #[error("Ticket not found for this ID")]
    TicketNotFound,
    #[error("Bet amount is too high. Maximum allowed bet is {0}")]
    BetTooHigh(Uint128),
    #[error("A game is already in progress for this week")]
    GameInProgress,
    #[error("Ticket not owned by you")]
    TicketNotOwnedByUser,
    #[error("You have already purchased {0}")]
    TicketAlreadyPurchased(Ticket)
}

impl From<LotteryError> for StdError {
    fn from(error: LotteryError) -> Self {
        StdError::generic_err(format!("{}", error))
    }
}