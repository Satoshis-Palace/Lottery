import { SecretNetworkClient, Wallet } from 'secretjs';
import dotenv from 'dotenv';
import { Contract, ContractInfo, getChainId, getMnemonic, getRpcUrl, readContractCode } from 'secretcontract';
import { LotteryContract, LotteryContractFactory } from '../contract';
import { InstantiateMsg, Config } from '../initMsg';


dotenv.config();

const CONTRACT_CODE_PATH = "./contract.wasm.gz";
const snip20info: ContractInfo = {
    code_hash: "ef8fd7734b3d8a2f5372836955c73e41d111303576857efa8e2c8c898f1c906c",
    address: "secret1lfq3zvlvhrl3nlx074fcwrxdxlyhapt7gssxw4"
}
describe('Counter Contract Lifecycle', () => {
    let secretNetworkClient: SecretNetworkClient;
    let LotteryContract: LotteryContract;
    beforeAll(async () => {
        // Initialize SecretNetworkClient with environment variables
        const mnemonic = getMnemonic();
        const chainId = getChainId();
        const rpcUrl = getRpcUrl();
        secretNetworkClient = new SecretNetworkClient({
            chainId: chainId,
            url: rpcUrl,
            wallet: new Wallet(mnemonic),
            walletAddress: new Wallet(mnemonic).address
        });

        // Read contract wasm
        const contractWasm = readContractCode(CONTRACT_CODE_PATH);
        const config: Config = {
            amount: '100000',
            cost: '10',
            length: '7',
            difficulty: '3'
        }

        // Instantiate CounterContract
        const initMsg: InstantiateMsg = { config: config };
        LotteryContract = await LotteryContractFactory.createLotteryContract(secretNetworkClient, initMsg, contractWasm, snip20info);
    });
    test('Contract should be uploaded and instantiated', () => {
        expect(LotteryContract.getContractAddress()).toBeTruthy();
        console.log(LotteryContract.getContractAddress())
        console.log(LotteryContract.getContractCodeHash())
        // Additional assertions as needed
    });

    test('Register tests', async () => { //TODO also send the staring amount of snip (100000) or remove it
        let reply = await LotteryContract.registerSnip20(snip20info.address, snip20info.code_hash)

        let check = await LotteryContract.LotteryQuery({ get_snip: { id: 0 } })
        let check2 = await LotteryContract.getLotterySnip20(0)
        console.log(check)
        console.log(check2)
        //@ts-ignore
        console.log(check.address.contract.contract_info)
    });

    test('Buy ticket test', async () => {
        var ticket = { numbers: ["1", "2", "3"] }
        console.log(await LotteryContract.buyTickets([ticket], "secret160se29szxttl0xufrm2qwjquszl87px0ls46y6"))
        console.log(secretNetworkClient.address)
        //set key nvoierbnv32098239cn29r23
        console.log(await LotteryContract.setViewKey("nvoierbnv32098239cn29r23"))
        console.log(await LotteryContract.getUsersTickets("secret160se29szxttl0xufrm2qwjquszl87px0ls46y6", 1, "nvoierbnv32098239cn29r23"))
        console.log(await LotteryContract.getTicketsUsers(ticket, 1))
    });

    test('Buy tickets test', async () => {
        var ticket1 = { numbers: ["1", "2", "4"] }
        var ticket2 = { numbers: ["2", "2", "4"] }
        var ticket3 = { numbers: ["3", "2", "4"] }
        var ticket4 = { numbers: ["4", "2", "4"] }
        var ticket5 = { numbers: ["5", "2", "4"] }
        var ticket6 = { numbers: ["6", "2", "4"] }
        var ticket7 = { numbers: ["7", "2", "4"] }
        var ticket8 = { numbers: ["8", "2", "4"] }
        var tickets = [ticket1, ticket2, ticket3, ticket4, ticket5]
        console.log(await LotteryContract.buyTickets(tickets, "secret160se29szxttl0xufrm2qwjquszl87px0ls46y6"))
        console.log(secretNetworkClient.address)
        console.log(await LotteryContract.setViewKey("nvoierbnv32098239cn29r232352356"))
        console.log(await LotteryContract.getUsersTickets("secret160se29szxttl0xufrm2qwjquszl87px0ls46y6", 1, "nvoierbnv32098239cn29r232352356"))
        console.log(await LotteryContract.getTicketsUsers(ticket1, 1))
        console.log(await LotteryContract.getTicketsUsers(ticket2, 1))
        console.log(await LotteryContract.getTicketsUsers(ticket3, 1))
        console.log(await LotteryContract.getTicketsUsers(ticket4, 1))
        console.log(await LotteryContract.getTicketsUsers(ticket5, 1))
        console.log(await LotteryContract.getBatchTicketsUsers([ticket1, ticket2, ticket3, ticket4, ticket5, ticket6, ticket7, ticket8], 1))        
    });

    test('View functions test', async () => {
        console.log(await LotteryContract.getTotalMoneyCollected())
        console.log(await LotteryContract.getUsersTotalTickets("secret160se29szxttl0xufrm2qwjquszl87px0ls46y6", "nvoierbnv32098239cn29r232352356"))
        console.log(await LotteryContract.pullLottery())
        console.log(await LotteryContract.getBatchLotteryInfo(1,2))
    });

    // test('Pull lottery', async () => {
    //     var prom = await LotteryContract.pullLottery()
    //     console.log(prom)
    // });

    test('Print out info again', async () => {
        console.log(LotteryContract.getContractAddress())
        console.log(LotteryContract.getContractCodeHash())
    });
});