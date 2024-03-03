import { Ticket } from "./executeMsg";

// Query message types for the contract
export type QueryMsg =
    | { get_users_tickets: { address: string; lottery_id: number; key:string } } //TODO remove this maybe (redundent now)
    | { get_tickets_user: { ticket: Ticket; lottery_id: number } }   //TODO remove this maybe (redundent now)
    | { get_lottery: { id: number } }
    | { get_batch_lottery: {start_id: number, end_id: number} }
    | { get_total_money_collected: {} }
    | { get_user_total_tickets: {address: string, key:string} }
    | GetLatestLottery
    | GetTicketPrice
    | GetSnip
    | GetUsersTickets
    | GetTicketsUser
    | GetBatchTicketsUser
    | GetAdmin;


export interface GetLatestLottery{
    get_latest_lottery: {}
}
export interface GetTicketPrice{
    get_ticket_price: {id: number}
}
export interface GetSnip{
    get_snip: {id: number}
}
export interface GetUsersTickets{
    get_users_tickets: {address: string, lottery_id: number, key:string}
}
export interface GetTicketsUser{
    get_tickets_user: {ticket: Ticket, lottery_id: number}
}
export interface GetBatchTicketsUser{
    batch_get_tickets_user: {tickets: Ticket[], lottery_id: number}
}
export interface GetAdmin{
    get_owner: {}
}




//Responses
export type QueryAnswer =
    | LatestLotteryResponse
    | TicketPriceResponse
    | AddressResponse
    //| UsersTicketsResponse
    | Ticket[]
    | LotteryResponse
    | string;

export interface LatestLotteryResponse{
    id : number
}
export interface TicketPriceResponse{
    cost: number
}
export interface AddressResponse{
    address: string 
}

export interface LotteryResponse{
    difficulty_num: number,
    cost: number,
    start_time: number,
    end_time: number,
    numbers: string[], // Changed to Vec<Uint64>
    is_redeemed: boolean,
    amount: number, // Amount of money for the lottery
    length: number,  // Length of the lottery
}

export interface LotteryResponse{
    difficulty_num: number,
    cost: number,
    start_time: number,
    numbers: string[], // Changed to Vec<Uint64>
    is_redeemed: boolean,
    amount: number, // Amount of money for the lottery
    length: number,  // Length of the lottery
}