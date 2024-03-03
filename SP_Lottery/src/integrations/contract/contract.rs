use cosmwasm_std::{
    to_binary, from_binary, QuerierWrapper, StdResult, WasmQuery, QueryRequest, 
    ContractInfo
};
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Contract {
    pub contract_info: ContractInfo,
}

impl Contract {
    pub fn new(contract_info: ContractInfo) -> Self {
        Contract { contract_info }
    }

    pub fn query<Q: Serialize, R: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
        query_msg: &Q,
    ) -> StdResult<R> {
        let wasm_query = WasmQuery::Smart {
            contract_addr: self.contract_info.address.to_string(),
            msg: to_binary(query_msg)?,
            code_hash: self.contract_info.code_hash.clone(),
        };
        let query_request = QueryRequest::Wasm(wasm_query);

        let binary_response = querier.query(&query_request)?;

        let response: R = from_binary(&binary_response)?;
        Ok(response)
    }
}
