#[cfg(test)]
mod tests {
    // ... existing imports ...

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, MockApi, MockQuerier}, Addr, Binary, BlockInfo, Env, OwnedDeps, Timestamp, Uint128, Uint64
    };

    use crate::{
        lottery::{
            actions::{try_buy_ticket_multi, try_pull_numbers_and_increment}, lottery::{get_lottery, get_lottery_count, get_total_money_collected, redeem_ticket, save_lottery, Lottery}, tests::instantiate_test::tests::{_initialize_test, _regrister_snip20}, ticket::Ticket
        }}
    ;

    
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
    fn get_first_lottery_test() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);
        let lotery_info = get_lottery(&deps.storage, 1).unwrap();

        let expected_lottery = Lottery {
            difficulty_num: Uint64::new(3),
            cost: Uint128::new(10),
            start_time: Uint64::new(0),
            end_time: Uint64::new(0), //TODO maybe check this
            numbers: Vec::new(),
            is_redeemed: false,
            amount: Uint128::new(100_000),
            length: Uint64::new(1),
            tickets_sold: Uint64::new(0),
        };

        assert_eq!(expected_lottery.difficulty_num, lotery_info.difficulty_num);
        assert_eq!(expected_lottery.cost, lotery_info.cost);
        assert_eq!(expected_lottery.numbers, lotery_info.numbers);
        assert_eq!(expected_lottery.is_redeemed, lotery_info.is_redeemed);
        assert_eq!(expected_lottery.amount, lotery_info.amount);
        assert_eq!(expected_lottery.length, lotery_info.length);
        assert_ne!(expected_lottery.start_time, lotery_info.start_time);
        assert_eq!(expected_lottery.tickets_sold, lotery_info.tickets_sold);
    }
    #[test]
    fn get_global_stats_lottery_test() {
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

        let data = get_total_money_collected(&mut deps.storage).load();
        let response = data.unwrap();
        assert_eq!(response, Uint128::new(150));

        //Pull lottery manualy
        let high_time_env = mock_env_with_time(2_000_000_000);
        let _ = try_pull_numbers_and_increment(&mut deps.as_mut(), high_time_env);

        let data = get_total_money_collected(&mut deps.storage).load();
        let response = data.unwrap();
        assert_eq!(response, Uint128::new(150));

        //Modify the winnings numbers 
        let mut lottery_info = get_lottery(&deps.storage, 1).unwrap();
        lottery_info.numbers = ticket1.numbers.clone();
        let _result = save_lottery(&mut deps.storage, 1, lottery_info);
        let _modified_lottery_info = get_lottery(&deps.storage, 1).unwrap();

        // Claim all winnings
        let _ = redeem_ticket(&mut deps.as_mut(), address, ticket1.clone(), 1).unwrap();
        let _ = redeem_ticket(&mut deps.as_mut(), address2, ticket1.clone(), 1).unwrap();
        let _ = redeem_ticket(&mut deps.as_mut(), address3, ticket1.clone(), 1).unwrap();


        let data = get_total_money_collected(&mut deps.storage).load();
        let response = data.unwrap();
        assert_eq!(response, Uint128::new(150));
    }

    // ... other test functions ...
}
