#[cfg(test)]
mod tests {
    // ... existing imports ...

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        Addr, Binary, BlockInfo, Env, Timestamp, Uint128, Uint64,
    };

    use crate::
        lottery::{
            actions::{
                try_pull_numbers_and_increment,
                try_pull_numbers_and_increment_admin,
            },
            lottery::{get_lottery, get_lottery_count, Lottery},
            tests::instantiate_test::tests::{_initialize_test, _regrister_snip20},
        }
    ;

    const SECONDS_PER_DAY: u64 = 86_400;
    fn mock_env_with_block_time(time_seconds: u64) -> Env {
        let mut env = mock_env();
        let _random_bytes: [u8; 32] = [0; 32]; // Example fixed array of 32 bytes

        env.block = BlockInfo {
            height: env.block.height,
            time: Timestamp::from_seconds(time_seconds), // Set to specific time in seconds
            chain_id: env.block.chain_id,
            random: Some(Binary::from("Test".as_bytes())), // Here, we're using a fixed byte array
        };
        env
    }

    #[test]
    fn update_lottery_test() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let address = Addr::unchecked("creator"); // Using a mock address for testing

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Update lottery
        let difficulty_num = Uint64::new(3);
        let length = Uint64::new(2);
        let cost = Uint128::new(100);
        let high_block_height_env =
            mock_env_with_block_time(env.block.time.seconds() + SECONDS_PER_DAY * 7);
        let loto_count_before = get_lottery_count(&deps.storage).load();
        let _result = try_pull_numbers_and_increment_admin(
            &mut deps.as_mut(),
            high_block_height_env.clone(),
            difficulty_num,
            length,
            cost,
            address,
        )
        .unwrap();
        let loto_count_after = get_lottery_count(&deps.storage).load();

        let expected_lottery = Lottery {
            difficulty_num: Uint64::new(3),
            cost: Uint128::new(10),
            start_time: Uint64::new(0), // Assuming this remains unchanged after update
            end_time: Uint64::new(0), //TODO maybe check this
            numbers: Vec::new(),        // Assuming default value
            is_redeemed: false,
            amount: Uint128::new(100_000), // Assuming this remains unchanged after update
            length: Uint64::new(1),
            tickets_sold: Uint64::new(0),
        };

        // Retrieve lottery information of the old lottery and verify old values
        assert_eq!(1, loto_count_before.unwrap());
        assert_eq!(2, loto_count_after.unwrap());
        let lottery_info = get_lottery(&deps.storage, 1).unwrap();
        assert_eq!(expected_lottery.difficulty_num, lottery_info.difficulty_num);
        assert_eq!(expected_lottery.cost, lottery_info.cost);
        //assert_eq!(expected_lottery.numbers, lottery_info.numbers);
        assert_ne!(expected_lottery.numbers, lottery_info.numbers);
        assert_eq!(expected_lottery.is_redeemed, lottery_info.is_redeemed);
        assert_eq!(expected_lottery.amount, lottery_info.amount);
        assert_eq!(expected_lottery.length, lottery_info.length);
        assert_ne!(expected_lottery.start_time, lottery_info.start_time);
        assert_eq!(expected_lottery.tickets_sold, lottery_info.tickets_sold);

        // Retrieve updated lottery information of the new lottery created and its updated values
        let lottery_info = get_lottery(&deps.storage, 2).unwrap();
        assert_eq!(expected_lottery.difficulty_num, lottery_info.difficulty_num);
        assert_eq!(Uint128::new(100), lottery_info.cost);
        assert_eq!(expected_lottery.numbers, lottery_info.numbers);
        assert_eq!(expected_lottery.is_redeemed, lottery_info.is_redeemed);
        assert_eq!(Uint128::new(0), lottery_info.amount);
        assert_eq!(Uint64::new(2), lottery_info.length);
        assert_ne!(expected_lottery.start_time, lottery_info.start_time);
        assert_eq!(expected_lottery.tickets_sold, lottery_info.tickets_sold);
    }
    #[test]
    fn pull_lottery_numbers_test() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Update lottery
        let _difficulty_num = Uint64::new(3);
        let _length = Uint64::new(2);
        let _cost = Uint128::new(100);
        let high_block_height_env =
            mock_env_with_block_time(env.block.time.seconds() + SECONDS_PER_DAY * 7);
        let _result =
            try_pull_numbers_and_increment(&mut deps.as_mut(), high_block_height_env.clone())
                .unwrap();

        // Retrieve updated lottery information
        let lottery_info = get_lottery(&deps.storage, 2).unwrap();

        let expected_lottery = Lottery {
            difficulty_num: Uint64::new(3),
            cost: Uint128::new(10),
            start_time: Uint64::new(0), // Assuming this remains unchanged after update
            end_time: Uint64::new(0), //TODO maybe check this
            numbers: Vec::new(),        // Assuming default value
            is_redeemed: false,
            amount: Uint128::new(0), // Assuming this remains unchanged after update
            length: Uint64::new(1),
            tickets_sold: Uint64::new(0),
        };

        assert_eq!(expected_lottery.difficulty_num, lottery_info.difficulty_num);
        assert_eq!(expected_lottery.cost, lottery_info.cost);
        assert_eq!(expected_lottery.numbers, lottery_info.numbers);
        assert_eq!(expected_lottery.is_redeemed, lottery_info.is_redeemed);
        assert_eq!(expected_lottery.amount, lottery_info.amount);
        assert_eq!(expected_lottery.length, lottery_info.length);
        assert_ne!(expected_lottery.start_time, lottery_info.start_time)
    }
    #[test]
    fn pull_lottery_numbers_multi_test() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let high_block_height_env =
            mock_env_with_block_time(env.block.time.seconds() + SECONDS_PER_DAY * 7);
        // Initialize the contract
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);

        // Update lottery
        let _result =
            try_pull_numbers_and_increment(&mut deps.as_mut(), high_block_height_env.clone())
                .unwrap();

        let expected_lottery = Lottery {
            difficulty_num: Uint64::new(3),
            cost: Uint128::new(10),
            start_time: Uint64::new(0), // Assuming this remains unchanged after update
            end_time: Uint64::new(0), //TODO maybe check this
            numbers: Vec::new(),        // Assuming default value
            is_redeemed: false,
            amount: Uint128::new(0), // Assuming this remains unchanged after update
            length: Uint64::new(1),
            tickets_sold: Uint64::new(0),
        };

        // Retrieve updated lottery information
        let lottery_info = get_lottery(&deps.storage, 1).unwrap();
        assert_eq!(expected_lottery.difficulty_num, lottery_info.difficulty_num);
        assert_eq!(expected_lottery.cost, lottery_info.cost);
        assert_ne!(expected_lottery.numbers, lottery_info.numbers);
        //assert_eq!(expected_lottery.numbers, lottery_info.numbers);
        assert_eq!(expected_lottery.is_redeemed, lottery_info.is_redeemed);
        assert_eq!(Uint128::new(100_000), lottery_info.amount);
        assert_eq!(expected_lottery.length, lottery_info.length);
        assert_ne!(expected_lottery.start_time, lottery_info.start_time);
        assert_eq!(expected_lottery.tickets_sold, lottery_info.tickets_sold);

        let high_block_height_env1 =
            mock_env_with_block_time(env.block.time.seconds() + SECONDS_PER_DAY * 14);
        let _result =
            try_pull_numbers_and_increment(&mut deps.as_mut(), high_block_height_env1.clone())
                .unwrap();

        let lottery_info = get_lottery(&deps.storage, 2).unwrap();
        assert_eq!(expected_lottery.difficulty_num, lottery_info.difficulty_num);
        assert_eq!(expected_lottery.cost, lottery_info.cost);
        assert_ne!(expected_lottery.numbers, lottery_info.numbers);
        // assert_eq!(expected_lottery.numbers, lottery_info.numbers);
        assert_eq!(expected_lottery.is_redeemed, lottery_info.is_redeemed);
        assert_eq!(Uint128::new(100_000), lottery_info.amount);
        assert_eq!(expected_lottery.length, lottery_info.length);
        assert_ne!(expected_lottery.start_time, lottery_info.start_time);
        assert_eq!(expected_lottery.tickets_sold, lottery_info.tickets_sold);

        let lottery_info = get_lottery(&deps.storage, 3).unwrap();
        assert_eq!(expected_lottery.difficulty_num, lottery_info.difficulty_num);
        assert_eq!(expected_lottery.cost, lottery_info.cost);
        assert_eq!(expected_lottery.numbers, lottery_info.numbers);
        assert_eq!(expected_lottery.is_redeemed, lottery_info.is_redeemed);
        assert_eq!(Uint128::new(0), lottery_info.amount); //Most recent lottery has 0 amount but previous has all the amount.
        assert_eq!(expected_lottery.length, lottery_info.length);
        assert_ne!(expected_lottery.start_time, lottery_info.start_time);
        assert_eq!(expected_lottery.tickets_sold, lottery_info.tickets_sold);
    }
}
