#[cfg(test)]
mod tests {

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        Addr, Binary, BlockInfo, Env, Timestamp, Uint128, Uint64,
    };

    use crate::lottery::{
        actions::{try_buy_ticket_multi, try_pull_numbers_and_increment},
        lottery::{
            get_lottery, get_lottery_count, is_winning_ticket, redeem_ticket, save_lottery, Lottery,
        },
        tests::instantiate_test::tests::{_initialize_test, _regrister_snip20},
        ticket::Ticket,
    };

pub fn mock_env_with_time(time: u64) -> Env {
        let mut env = mock_env();
        let _random_bytes: [u8; 32] = [0; 32]; // Example fixed array of 32 bytes

        env.block = BlockInfo {
            height: env.block.height,
            time: Timestamp::from_seconds(time),
            chain_id: env.block.chain_id,
            random: Some(Binary::from("Test".as_bytes())), // Here, we're using a fixed byte array
        };
        return env;
    }

    #[test]
    fn buy_single_ticket_and_lose_lottery_test() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        let ticket1 = Ticket {
            numbers: vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)],
        };

        // Attempt to buy a ticket
        let amount = Uint128::new(10);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Assuming try_buy_ticket takes DepsMut, address (Option<Addr>), amount (Option<Uint128>), and ticket (Ticket)
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address.clone()),
            amount,
            vec![ticket1.clone()],
        )
        .unwrap();

        //Pull lottery manualy
        let high_time_env = mock_env_with_time(2_000_000_000);
        let _results0 = try_pull_numbers_and_increment(&mut deps.as_mut(), high_time_env);
        let result = is_winning_ticket(&mut deps.storage, &ticket1, 1).unwrap();
        assert_eq!(false, result); //Not a winning ticket

        let result = redeem_ticket(&mut deps.as_mut(), address, ticket1, 1)
            .unwrap_err()
            .to_string();

        // Assert that the message matches the expected text
        assert_eq!("Generic error: Ticket is not a winner", result);
    }
    #[test]
    fn buy_single_ticket_and_win_lottery_test() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        let ticket1 = Ticket { numbers: vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)] };

        // Attempt to buy a ticket
        let amount = Uint128::new(10);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Assuming try_buy_ticket takes DepsMut, address (Option<Addr>), amount (Option<Uint128>), and ticket (Ticket)
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address.clone()),
            amount,
            vec![ticket1.clone()],
        )
        .unwrap();

        let _expected_lottery = Lottery {
            difficulty_num: Uint64::new(3),
            cost: Uint128::new(10),
            start_time: Uint64::new(0), // Assuming this remains unchanged after update
            end_time: Uint64::new(0), //TODO maybe check this
            numbers: ticket1.numbers.clone(), // Assuming default value
            is_redeemed: false,
            amount: Uint128::new(100_010), // Assuming this remains unchanged after update
            length: Uint64::new(1),
            tickets_sold: Uint64::new(1),
        };

        let high_time_env = mock_env_with_time(2_000_000_000);
        //Pull lottery manualy
        let _results0 = try_pull_numbers_and_increment(&mut deps.as_mut(), high_time_env);

        let mut lottery_info = get_lottery(&deps.storage, 1).unwrap();

        //Modify the winnings numbers
        lottery_info.numbers = ticket1.numbers.clone();
        let _result = save_lottery(&mut deps.storage, 1, lottery_info);

        let result = is_winning_ticket(&mut deps.storage, &ticket1, 1).unwrap();
        assert_eq!(true, result);
        //Ticket is indeed winner

        let result = redeem_ticket(&mut deps.as_mut(), address, ticket1, 1).unwrap();
        // Find the specific attribute in the response
        let won_amount_message = result
            .attributes
            .into_iter()
            .find(|attr| attr.key == "message")
            .map(|attr| attr.value)
            .unwrap_or_default();

        // Assert that the message matches the expected text
        assert_eq!("You won 100010 amount", won_amount_message.as_str());
    }
    #[test]
    fn buy_multi_ticket_and_win_lottery_test() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        let ticket1 = Ticket {
            numbers: vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)],
        };
        let ticket2 = Ticket {
            numbers: vec![Uint64::new(2), Uint64::new(2), Uint64::new(3)],
        };
        let ticket3 = Ticket {
            numbers: vec![Uint64::new(3), Uint64::new(2), Uint64::new(3)],
        };
        let ticket4 = Ticket {
            numbers: vec![Uint64::new(4), Uint64::new(2), Uint64::new(3)],
        };
        let ticket5 = Ticket {
            numbers: vec![Uint64::new(5), Uint64::new(2), Uint64::new(3)],
        };

        // Attempt to buy a ticket
        let amount_5 = Uint128::new(50);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Assuming try_buy_ticket takes DepsMut, address (Option<Addr>), amount (Option<Uint128>), and ticket (Ticket)
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address.clone()),
            amount_5,
            vec![
                ticket1.clone(),
                ticket2.clone(),
                ticket3.clone(),
                ticket4.clone(),
                ticket5.clone(),
            ],
        )
        .unwrap();

        let expected_lottery = Lottery {
            difficulty_num: Uint64::new(3),
            cost: Uint128::new(10),
            start_time: Uint64::new(0), // Assuming this remains unchanged after update
            end_time: Uint64::new(0), //TODO maybe check this
            numbers: ticket1.numbers.clone(), // Assuming default value
            is_redeemed: false,
            amount: Uint128::new(100_050), // Assuming this remains unchanged after update
            length: Uint64::new(1),
            tickets_sold: Uint64::new(5),
        };

        let high_time_env = mock_env_with_time(2_000_000_000);
        //Pull lottery manualy
        let _results0 = try_pull_numbers_and_increment(&mut deps.as_mut(), high_time_env);
        // let result1 = pull_lottery_numbers(high_block_height_env.clone(), &mut deps.storage);
        // // Call increment_lottery with the correct DepsMut reference
        // let result2 = increment_lottery(mock_env(), &mut deps_mut);

        //Pull lottery automaticaly
        //let result3 = try_pull_numbers_and_increment(&mut deps.as_mut(),high_block_height_env.clone()).unwrap();
        // Retrieve updated lottery information
        let mut lottery_info = get_lottery(&deps.storage, 1).unwrap();

        //Modify the winnings numbers
        lottery_info.numbers = ticket1.numbers.clone();
        let _result = save_lottery(&mut deps.storage, 1, lottery_info);

        let modified_lottery_info = get_lottery(&deps.storage, 1).unwrap();

        assert_eq!(
            expected_lottery.difficulty_num,
            modified_lottery_info.difficulty_num
        );
        assert_eq!(expected_lottery.cost, modified_lottery_info.cost);
        assert_eq!(expected_lottery.numbers, modified_lottery_info.numbers);
        assert_eq!(
            expected_lottery.is_redeemed,
            modified_lottery_info.is_redeemed
        );
        assert_eq!(expected_lottery.amount, modified_lottery_info.amount);
        assert_eq!(expected_lottery.length, modified_lottery_info.length);
        assert_ne!(
            expected_lottery.start_time,
            modified_lottery_info.start_time
        );
        assert_eq!(
            expected_lottery.tickets_sold,
            modified_lottery_info.tickets_sold
        );
        //Successfully modified lottery

        let result = is_winning_ticket(&mut deps.storage, &ticket1, 1).unwrap();
        assert_eq!(true, result);
        //Ticket is indeed winner

        let result = redeem_ticket(&mut deps.as_mut(), address, ticket1, 1).unwrap();
        // Find the specific attribute in the response
        let won_amount_message = result
            .attributes
            .into_iter()
            .find(|attr| attr.key == "message")
            .map(|attr| attr.value)
            .unwrap_or_default();

        // Assert that the message matches the expected text
        assert_eq!("You won 100050 amount", won_amount_message.as_str());
    }
    #[test]
    fn buy_multi_ticket_with_multi_winner_and_weird_math_lottery_test() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        let ticket1 = Ticket {
            numbers: vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)],
        };
        let ticket2 = Ticket {
            numbers: vec![Uint64::new(2), Uint64::new(2), Uint64::new(3)],
        };
        let ticket3 = Ticket {
            numbers: vec![Uint64::new(3), Uint64::new(2), Uint64::new(3)],
        };
        let ticket4 = Ticket {
            numbers: vec![Uint64::new(4), Uint64::new(2), Uint64::new(3)],
        };
        let ticket5 = Ticket {
            numbers: vec![Uint64::new(5), Uint64::new(2), Uint64::new(3)],
        };

        // Attempt to buy a ticket
        let amount_5 = Uint128::new(50);
        let amount = Uint128::new(10);
        let address = Addr::unchecked("sender"); // Using a mock address for testing
        let address2 = Addr::unchecked("sender2"); // Using a mock address for testing
        let address3 = Addr::unchecked("sender3"); // Using a mock address for testing

        // Get the current lottery ID (assuming this is how you get it)
        let current_lottery_id = match get_lottery_count(&mut deps.storage).load() {
            Ok(value) => value,
            Err(_) => 0, // Replace this with the default value you want to use in case of an error
        };

        let _expected_lottery_id = if current_lottery_id > 0 {
            current_lottery_id
        } else {
            0
        };

        // Assuming try_buy_ticket takes DepsMut, address (Option<Addr>), amount (Option<Uint128>), and ticket (Ticket)
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address.clone()),
            amount_5,
            vec![
                ticket1.clone(),
                ticket2.clone(),
                ticket3.clone(),
                ticket4.clone(),
                ticket5.clone(),
            ],
        )
        .unwrap();
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address2.clone()),
            amount,
            vec![ticket1.clone()],
        )
        .unwrap();
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address3.clone()),
            amount,
            vec![ticket1.clone()],
        )
        .unwrap();

        let high_time_env = mock_env_with_time(2_000_000_000);
        //Pull lottery manualy
        let _ = try_pull_numbers_and_increment(&mut deps.as_mut(), high_time_env);
        let mut lottery_info = get_lottery(&deps.storage, 1).unwrap();

        //Modify the winnings numbers
        lottery_info.numbers = ticket1.numbers.clone();
        let _result = save_lottery(&mut deps.storage, 1, lottery_info);

        let _modified_lottery_info = get_lottery(&deps.storage, 1).unwrap();

        let result = is_winning_ticket(&mut deps.storage, &ticket1, 1).unwrap();
        assert_eq!(true, result);
        //Ticket is indeed winner

        let result = redeem_ticket(&mut deps.as_mut(), address, ticket1.clone(), 1).unwrap();
        // Find the specific attribute in the response
        let won_amount_message = result
            .attributes
            .into_iter()
            .find(|attr| attr.key == "message")
            .map(|attr| attr.value)
            .unwrap_or_default();

        // Assert that the message matches the expected text
        assert_eq!("You won 33356 amount", won_amount_message.as_str());

        let result2 = redeem_ticket(&mut deps.as_mut(), address2, ticket1.clone(), 1).unwrap();
        // Find the specific attribute in the response
        let won_amount_message = result2
            .attributes
            .into_iter()
            .find(|attr| attr.key == "message")
            .map(|attr| attr.value)
            .unwrap_or_default();

        // Assert that the message matches the expected text
        assert_eq!("You won 33356 amount", won_amount_message.as_str());
        let result3 = redeem_ticket(&mut deps.as_mut(), address3, ticket1.clone(), 1).unwrap();
        // Find the specific attribute in the response
        let won_amount_message = result3
            .attributes
            .into_iter()
            .find(|attr| attr.key == "message")
            .map(|attr| attr.value)
            .unwrap_or_default();

        // Assert that the message matches the expected text
        assert_eq!("You won 33356 amount", won_amount_message.as_str());
    }
    #[test]
    fn buy_multi_ticket_with_multi_winner_and_weird_math_lottery_test2() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        let ticket1 = Ticket {
            numbers: vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)],
        };
        let ticket2 = Ticket {
            numbers: vec![Uint64::new(2), Uint64::new(2), Uint64::new(3)],
        };
        let ticket3 = Ticket {
            numbers: vec![Uint64::new(3), Uint64::new(2), Uint64::new(3)],
        };
        let ticket4 = Ticket {
            numbers: vec![Uint64::new(4), Uint64::new(2), Uint64::new(3)],
        };
        let ticket5 = Ticket {
            numbers: vec![Uint64::new(5), Uint64::new(2), Uint64::new(3)],
        };

        // Attempt to buy a ticket
        let amount_5 = Uint128::new(50);
        let address = Addr::unchecked("sender"); // Using a mock address for testing
        let address2 = Addr::unchecked("sender2"); // Using a mock address for testing
        let address3 = Addr::unchecked("sender3"); // Using a mock address for testing

        // Get the current lottery ID (assuming this is how you get it)
        let current_lottery_id = match get_lottery_count(&mut deps.storage).load() {
            Ok(value) => value,
            Err(_) => 0, // Replace this with the default value you want to use in case of an error
        };

        let _expected_lottery_id = if current_lottery_id > 0 {
            current_lottery_id
        } else {
            0
        };

        // Assuming try_buy_ticket takes DepsMut, address (Option<Addr>), amount (Option<Uint128>), and ticket (Ticket)
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address.clone()),
            amount_5,
            vec![
                ticket1.clone(),
                ticket2.clone(),
                ticket3.clone(),
                ticket4.clone(),
                ticket5.clone(),
            ],
        )
        .unwrap();
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address2.clone()),
            amount_5,
            vec![
                ticket1.clone(),
                ticket2.clone(),
                ticket3.clone(),
                ticket4.clone(),
                ticket5.clone(),
            ],
        )
        .unwrap();
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address3.clone()),
            amount_5,
            vec![
                ticket1.clone(),
                ticket2.clone(),
                ticket3.clone(),
                ticket4.clone(),
                ticket5.clone(),
            ],
        )
        .unwrap();

        //Pull lottery manualy
        let high_time_env = mock_env_with_time(2_000_000_000);
        let _ = try_pull_numbers_and_increment(&mut deps.as_mut(), high_time_env);
        let mut lottery_info = get_lottery(&deps.storage, 1).unwrap();

        //Modify the winnings numbers
        lottery_info.numbers = ticket1.numbers.clone();
        let _result = save_lottery(&mut deps.storage, 1, lottery_info);
        let _modified_lottery_info = get_lottery(&deps.storage, 1).unwrap();

        let result = is_winning_ticket(&mut deps.storage, &ticket1, 1).unwrap();
        assert_eq!(true, result);
        //Ticket is indeed winner

        let result = redeem_ticket(&mut deps.as_mut(), address, ticket1.clone(), 1).unwrap();
        // Find the specific attribute in the response
        let won_amount_message = result
            .attributes
            .into_iter()
            .find(|attr| attr.key == "message")
            .map(|attr| attr.value)
            .unwrap_or_default();

        // Assert that the message matches the expected text
        assert_eq!("You won 33383 amount", won_amount_message.as_str());

        let result2 = redeem_ticket(&mut deps.as_mut(), address2, ticket1.clone(), 1).unwrap();
        // Find the specific attribute in the response
        let won_amount_message = result2
            .attributes
            .into_iter()
            .find(|attr| attr.key == "message")
            .map(|attr| attr.value)
            .unwrap_or_default();

        // Assert that the message matches the expected text
        assert_eq!("You won 33383 amount", won_amount_message.as_str());
        let result3 = redeem_ticket(&mut deps.as_mut(), address3, ticket1.clone(), 1).unwrap();
        // Find the specific attribute in the response
        let won_amount_message = result3
            .attributes
            .into_iter()
            .find(|attr| attr.key == "message")
            .map(|attr| attr.value)
            .unwrap_or_default();

        // Assert that the message matches the expected text
        assert_eq!("You won 33383 amount", won_amount_message.as_str());
    }

    //TODO check for user that buys and wins a ticket then buys more tickets
    //TODO check for user that buys and wins a ticket then the lottery increments past the claim and they try to claim
}
