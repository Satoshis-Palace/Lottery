pub mod tests {
    use crate::{
        contract::instantiate,
        integrations::snip_20_manager::actions::try_register,
        msg::{Config, InstantiateMsg},
    };
    use cosmwasm_std::{
        coins, testing::{ mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, ContractInfo, Empty, OwnedDeps, Uint128, Uint64,
    };

    #[test]
    fn initialize() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();
        _initialize_test(&mut deps);
        _regrister_snip20(&mut deps);
    }

    pub fn _initialize_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>) {
        let msg = InstantiateMsg {
            config: Config {
                amount: Uint128::new(100_000),
                cost: Uint128::new(10),
                length: Uint64::new(1),
                difficulty: Uint64::new(3),
            },
        };

        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    pub fn _regrister_snip20(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>) {
        let info = mock_info("creator", &coins(1000, "earth"));

        // Mock SNIP-20 contract info
        let snip_20 = ContractInfo {
            address: Addr::unchecked("snip20_contract_address"), // Mock SNIP-20 address
            code_hash: "some_code_hash".to_string(),             // Mock code hash
        };

        // Mock entropy
        let entropy: [u8; 32] = [0; 32]; // Mock entropy, adjust as needed

        // Call try_register with the mock data
        let _res = try_register(&mut deps.as_mut(), mock_env(), info, snip_20, &entropy).unwrap();
    }
}
