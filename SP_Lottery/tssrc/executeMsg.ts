// Execute message types for the contract
export type ExecuteMsg =
    | { redeem_ticket: { ticket: Ticket; sender: string } }
    | { pull_lottery_numbers: {} }
    | { pull_lottery_numbers_admin: { difficulty_num: string; length: string; cost: string } }
    | RegisterMsg
    | { receive: { sender: string; from: string; amount: number; memo: string | null; msg: string } }
    | { redeem: { addr: string; hash: string; to: string; amount: number; denom: string | null } }
    | { set_viewing_key: {key: string}}
    | ClaimTicket;

    // TypeScript type for Ticket
export type Ticket = {
    numbers: Array<string>;
};
export interface ContractInfo {
    code_hash: String,
    address: String,
}
export type RegisterMsg = {
    register: {
        snip_20: ContractInfo; // Assuming ContractInfo is a defined TypeScript type
        entropy: string;       // Assuming Binary can be represented as a string
    };
};
export interface BuyTicketMsg {
    buy_ticket: {
        ticket: Ticket;
        sender?: string; // Assuming Option<Addr> can be represented as an optional string
    };
}
export interface BuyTicketMultiMsg {
    buy_ticket_multi: {
        tickets: Ticket[];
        sender?: string; // Assuming Option<Addr> can be represented as an optional string
    };
}
export interface ClaimTicket {
    redeem_ticket: {
        ticket: Ticket;
    };
}