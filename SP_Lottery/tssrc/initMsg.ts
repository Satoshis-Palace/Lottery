export interface InstantiateMsg {
     config: Config,
}

export interface Config {
    amount: string,
    cost: string,
    length: string,
    difficulty: string,
}