#[cfg(test)]
mod tests {
    // ... existing imports ...

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
    fn buy_ticket_and_check_amount_test() {
        let mut deps = mock_dependencies();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        let ticket = Ticket {
            numbers: vec![Uint64::new(1), Uint64::new(2), Uint64::new(3)],
        };

        // Attempt to buy a ticket
        let amount = Uint128::new(10);
        let address = Addr::unchecked("sender"); // Using a mock address for testing

        // Buy the ticket
        let _ = try_buy_ticket_multi(
            &mut deps.as_mut(),
            Some(address.clone()),
            amount,
            vec![ticket.clone()],
        )
        .unwrap();

        let expected_lottery = Lottery {
            difficulty_num: Uint64::new(3),
            cost: Uint128::new(10),
            start_time: Uint64::new(0), // Assuming this remains unchanged after update
            end_time: Uint64::new(0), //TODO maybe check this
            numbers: Vec::new(),        // Assuming default value
            is_redeemed: false,
            amount: Uint128::new(100_010), // Assuming this remains unchanged after update
            length: Uint64::new(1),
            tickets_sold: Uint64::new(1),
        };

        // Retrieve updated lottery information
        let lottery_info = get_lottery(&deps.storage, 1).unwrap();
        assert_eq!(expected_lottery.difficulty_num, lottery_info.difficulty_num);
        assert_eq!(expected_lottery.cost, lottery_info.cost);
        assert_eq!(expected_lottery.numbers, lottery_info.numbers);
        assert_eq!(expected_lottery.is_redeemed, lottery_info.is_redeemed);
        assert_eq!(expected_lottery.amount, lottery_info.amount);
        assert_eq!(expected_lottery.length, lottery_info.length);
        assert_ne!(expected_lottery.start_time, lottery_info.start_time);
        assert_eq!(expected_lottery.tickets_sold, lottery_info.tickets_sold);
    }
    #[test]
    fn buy_multi_ticket_and_check_amount_test() {
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
            numbers: Vec::new(),        // Assuming default value
            is_redeemed: false,
            amount: Uint128::new(100_050), // Assuming this remains unchanged after update
            length: Uint64::new(1),
            tickets_sold: Uint64::new(5),
        };

        // Retrieve updated lottery information
        let lottery_info = get_lottery(&deps.storage, 1).unwrap();
        assert_eq!(expected_lottery.difficulty_num, lottery_info.difficulty_num);
        assert_eq!(expected_lottery.cost, lottery_info.cost);
        assert_eq!(expected_lottery.numbers, lottery_info.numbers);
        assert_eq!(expected_lottery.is_redeemed, lottery_info.is_redeemed);
        assert_eq!(expected_lottery.amount, lottery_info.amount);
        assert_eq!(expected_lottery.length, lottery_info.length);
        assert_ne!(expected_lottery.start_time, lottery_info.start_time);
        assert_eq!(expected_lottery.tickets_sold, lottery_info.tickets_sold);
    }
    #[test]
    fn buy_multi_ticket_with_multi_player_and_check_amount_test() {
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

        // Pull lottery manualy
        let high_time_env = mock_env_with_time(2_000_000_000);
        let _ = try_pull_numbers_and_increment(&mut deps.as_mut(), high_time_env);

        let expected_lottery = Lottery {
            difficulty_num: Uint64::new(3),
            cost: Uint128::new(10),
            start_time: Uint64::new(0), // Assuming this remains unchanged after update
            end_time: Uint64::new(0), //TODO maybe check this
            numbers: Vec::new(),        // Assuming default value
            is_redeemed: false,
            amount: Uint128::new(100_150), // Assuming this remains unchanged after update
            length: Uint64::new(1),
            tickets_sold: Uint64::new(15),
        };

        // Retrieve updated lottery information
        let lottery_info = get_lottery(&deps.storage, 1).unwrap();
        assert_eq!(expected_lottery.difficulty_num, lottery_info.difficulty_num);
        assert_eq!(expected_lottery.cost, lottery_info.cost);
        assert_ne!(expected_lottery.numbers, lottery_info.numbers);
        assert_eq!(expected_lottery.is_redeemed, lottery_info.is_redeemed);
        assert_eq!(expected_lottery.amount, lottery_info.amount);
        assert_eq!(expected_lottery.length, lottery_info.length);
        assert_ne!(expected_lottery.start_time, lottery_info.start_time);
        assert_eq!(expected_lottery.tickets_sold, lottery_info.tickets_sold);
    }
}
