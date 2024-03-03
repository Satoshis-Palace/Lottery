#[cfg(test)]
mod tests {
    // ... existing imports ...

    use cosmwasm_std::{
        testing::mock_dependencies,
        Addr, StdError, Uint128, Uint64,
    };

    use crate::
        lottery::{
            actions::try_buy_ticket_multi, lottery::get_lottery_count, tests::instantiate_test::tests::{_initialize_test, _regrister_snip20}, ticket::{ get_tickets_user_pub, get_users_tickets, Ticket}
        }
    ;

    #[test]
    fn buy_ticket_test() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Define a ticket
        let ticket_numbers = vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket = Ticket {
            numbers: ticket_numbers,
        };

        // Attempt to buy a ticket
        let amount = Uint128::new(10);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Get the current lottery ID (assuming this is how you get it)
        let current_lottery_id = match get_lottery_count(&mut deps.storage).load() {
            Ok(value) => value,
            Err(_) => 0, // Replace this with the default value you want to use in case of an error
        };

        let expected_lottery_id = if current_lottery_id > 0 {
            current_lottery_id
        } else {
            0
        };

        // Buy the ticket
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address.clone()),
            amount,
            vec![ticket.clone()],  // Create a vector with a single ticket
        )
        .unwrap();

        // Retrieve the user's tickets for the specific lottery
        let users_tickets =
            get_users_tickets(deps.as_ref().storage, address, expected_lottery_id).unwrap();

        // Assert that the user's tickets contain the original ticket
        assert!(users_tickets.contains(&ticket));
    }

    #[test]
    fn buy_ticket_test_insufficient_price() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Define a ticket
        let ticket_numbers = vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket = Ticket {
            numbers: ticket_numbers,
        };

        // Attempt to buy a ticket
        let amount = Uint128::new(9);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Call try_buy_ticket and capture the result
        let result = try_buy_ticket_multi(&mut deps.as_mut(), Some(address), amount, vec![ticket.clone()]);

        // Assert that the result is an error with the specific message
        match result {
            Err(StdError::GenericErr { msg }) => {
                assert_eq!(msg, "Insufficient funds for buying tickets")
            }
            _ => panic!("Expected an insufficient amount error"),
        }
    }

    #[test]
    fn buy_ticket_test_overlysufficient_price() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Define a ticket
        let ticket_numbers = vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket = Ticket {
            numbers: ticket_numbers,
        };

        // Attempt to buy a ticket
        let amount = Uint128::new(11);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Call try_buy_ticket and capture the result
        let result = try_buy_ticket_multi(&mut deps.as_mut(), Some(address), amount, vec![ticket.clone()]);

        // Assert that the result is an error with the specific message
        match result {
            Err(StdError::GenericErr { msg }) => {
                assert_eq!(msg, "Overly sufficient funds for buying tickets")
            }
            _ => panic!("Expected an insufficient amount error"),
        }
    }
    #[test]
    fn buy_multi_ticket_test() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Define a ticket
        let ticket_numbers1 = vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket1 = Ticket {
            numbers: ticket_numbers1,
        };
        // Define a ticket
        let ticket_numbers2 = vec![Uint64::new(2), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket2 = Ticket {
            numbers: ticket_numbers2,
        };
        // Define a ticket
        let ticket_numbers3 = vec![Uint64::new(3), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket3 = Ticket {
            numbers: ticket_numbers3,
        };
        // Define a ticket
        let ticket_numbers4 = vec![Uint64::new(4), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket4 = Ticket {
            numbers: ticket_numbers4,
        };
        // Define a ticket
        let ticket_numbers5 = vec![Uint64::new(5), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket5 = Ticket {
            numbers: ticket_numbers5,
        };
        // Define a ticket
        let ticket_numbers6 = vec![Uint64::new(6), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket6 = Ticket {
            numbers: ticket_numbers6,
        };

        // Attempt to buy a ticket
        let amount_5 = Uint128::new(50);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Get the current lottery ID (assuming this is how you get it)
        let current_lottery_id = match get_lottery_count(&mut deps.storage).load() {
            Ok(value) => value,
            Err(_) => 0, // Replace this with the default value you want to use in case of an error
        };

        let expected_lottery_id = if current_lottery_id > 0 {
            current_lottery_id
        } else {
            0
        };

        // Assuming try_buy_ticket takes DepsMut, address (Option<Addr>), amount (Option<Uint128>), and ticket (Ticket)
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address.clone()),
            amount_5,
            vec![ticket1.clone(),ticket2.clone(),ticket3.clone(),ticket4.clone(),ticket5.clone()],
        )
        .unwrap();

        // Retrieve the user's tickets for the specific lottery
        let users_tickets =
            get_users_tickets(deps.as_ref().storage, address.clone(), expected_lottery_id).unwrap();

        // Assert that the first ticket in the user's tickets matches the original ticket
        assert_eq!(users_tickets.get(0), Some(&ticket1));
        assert_eq!(users_tickets.get(1), Some(&ticket2));
        assert_eq!(users_tickets.get(2), Some(&ticket3));
        assert_eq!(users_tickets.get(3), Some(&ticket4));
        assert_eq!(users_tickets.get(4), Some(&ticket5));

        let bought1 =
            get_tickets_user_pub(deps.as_ref().storage, &ticket1, expected_lottery_id).unwrap();
        let bought2 =
            get_tickets_user_pub(deps.as_ref().storage, &ticket2, expected_lottery_id).unwrap();
        let bought3 =
            get_tickets_user_pub(deps.as_ref().storage, &ticket3, expected_lottery_id).unwrap();
        let bought4 =
            get_tickets_user_pub(deps.as_ref().storage, &ticket4, expected_lottery_id).unwrap();
        let bought5 =
            get_tickets_user_pub(deps.as_ref().storage, &ticket5, expected_lottery_id).unwrap();
        let bought6 =
            get_tickets_user_pub(deps.as_ref().storage, &ticket6, expected_lottery_id).unwrap();
        assert_eq!(bought1, 1);
        assert_eq!(bought2, 1);
        assert_eq!(bought3, 1);
        assert_eq!(bought4, 1);
        assert_eq!(bought5, 1);
        assert_eq!(bought6, 0);
    }
    #[test]
    fn buy_existing_ticket_test() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Define a ticket
        let ticket_numbers1 = vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket1 = Ticket {
            numbers: ticket_numbers1,
        };
        // Define a ticket
        let ticket_numbers2 = vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)]; // Example ticket numbers
        let ticket2 = Ticket {
            numbers: ticket_numbers2,
        };

        // Attempt to buy a ticket
        let amount = Uint128::new(10);
        let address = Addr::unchecked("sender"); // Using a mock address for testinglet address = Addr::unchecked("sender"); // Using a mock address for testing

        // Assuming try_buy_ticket takes DepsMut, address (Option<Addr>), amount (Option<Uint128>), and ticket (Ticket)
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address.clone()),
            amount,
            vec![ticket1.clone()],
        )
        .unwrap();
        let result = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address.clone()),
            amount,
            vec![ticket2.clone()],
        );

        // Assert that the result is an error with the specific message
        match result {
            Err(StdError::GenericErr { msg }) => {
                assert_eq!(
                    msg,
                    "You have already purchased Ticket Numbers: [1, 2, 3]"
                )
            }
            _ => panic!("Expected an ticket purchase Error"),
        }
    }
    #[test]
    fn buy_ticket_too_many_numbers() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Define a ticket
        let ticket_numbers = vec![
            Uint64::new(1),
            Uint64::new(2),
            Uint64::new(3),
            Uint64::new(4),
        ]; // Example ticket numbers
        let ticket = Ticket {
            numbers: ticket_numbers,
        };

        // Attempt to buy a ticket
        let amount = Uint128::new(10);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Call try_buy_ticket and capture the result
        let result = try_buy_ticket_multi(&mut deps.as_mut(), Some(address), amount, vec![ticket.clone()]);

        // Assert that the result is an error with the specific message
        match result {
            Err(StdError::GenericErr { msg }) => {
                assert_eq!(
                    msg,
                    "Too many numbers on your lottery ticket: provided 4, but expected 3"
                )
            }
            _ => panic!("Expected a Too many numbers on your lottery ticket error"),
        }
    }
    #[test]
    fn buy_ticket_too_few_numbers() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Define a ticket
        let ticket_numbers = vec![Uint64::new(1), Uint64::new(2)]; // Example ticket numbers
        let ticket = Ticket {
            numbers: ticket_numbers,
        };

        // Attempt to buy a ticket
        let amount = Uint128::new(10);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Call try_buy_ticket and capture the result
        let result = try_buy_ticket_multi(&mut deps.as_mut(), Some(address), amount, vec![ticket.clone()]);

        // Assert that the result is an error with the specific message
        match result {
            Err(StdError::GenericErr { msg }) => {
                assert_eq!(
                    msg,
                    "Too few numbers on your lottery ticket: provided 2, but expected 3"
                )
            }
            _ => panic!("Expected a Too few numbers on your lottery ticket error"),
        }
    }
    #[test]
    fn buy_ticket_too_large_numbers() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Define a ticket
        let ticket_numbers = vec![Uint64::new(1), Uint64::new(2), Uint64::new(100)]; // Example ticket numbers
        let ticket = Ticket {
            numbers: ticket_numbers,
        };

        // Attempt to buy a ticket
        let amount = Uint128::new(10);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Call try_buy_ticket and capture the result
        let result = try_buy_ticket_multi(&mut deps.as_mut(), Some(address), amount, vec![ticket.clone()]);

        // Assert that the result is an error with the specific message
        match result {
            Err(StdError::GenericErr { msg }) => {
                assert_eq!(msg, "Ticket numbers must be between 0 and 99")
            }
            _ => panic!("Expected a Ticket numbers must be between 0 and 99 error"),
        }
    }
}
