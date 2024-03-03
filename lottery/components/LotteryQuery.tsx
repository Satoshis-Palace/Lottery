import { useState, useEffect, useContext } from "react";
import { LotteryGameLoaderProps } from "./LotteryGames";
import getLotteryContract, { DEFAULT_LOTTERY } from "../../../../../common/integrations/contracts/Lottery";
import { LotteryContext } from "../LotteryContext";
import { LotteryContract } from "sp_lottery";
import { SecretContext } from "../../../../../common/integrations/contracts/secretJs/SecretTSContext";

export default function LotteryQuery(): LotteryGameLoaderProps {
    const [lotteryGameLoaderProps, setLotteryGameLoaderProps] =
        useState<LotteryGameLoaderProps>({
            currentLotteryID: 0
        });
    const { setCurrentLotteryID, setLotteryMapping } = useContext(LotteryContext);
    const { secretjs, Refresh } = useContext(SecretContext);
    var currentLotteryID = 0
    var lastLotteryInfo = DEFAULT_LOTTERY
    var totalTicketsSold = 0
    var totalUsers = 0
    var lottery_contract: LotteryContract = getLotteryContract(secretjs)

    useEffect(() => {
        const fetchLottery = async () => { //TODO clean up
            currentLotteryID = await lottery_contract.getCurrentLotteryID()
            //totalTicketsSold = await lottery_contract.getTotalMoneyCollected
            setCurrentLotteryID(currentLotteryID)
            //TODO load all the lotteries in one go
            var allLotteries = await lottery_contract.getBatchLotteryInfo(1, currentLotteryID)
            allLotteries.forEach((lottery, index) => {
                // Since your index starts at 1, add 1 to the array index
                setLotteryMapping(index + 1, lottery);
                console.log("Running again")
            });
            //TODO get this id in less queriees. try to get it in the above lottery fetches
            setLotteryGameLoaderProps({
                currentLotteryID: currentLotteryID
            })
        }
        fetchLottery();
    }, [Refresh]); //used to have predictionInfoAr,

    return lotteryGameLoaderProps;
}
