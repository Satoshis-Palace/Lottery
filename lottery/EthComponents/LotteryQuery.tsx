import { useState, useEffect, useContext } from "react";
import { LotteryContext } from "../LotteryContext";
import { LotteryContract, LotteryResponse } from "sp_lottery";
import { SecretContext } from "../../../../../common/integrations/contracts/secretJs/SecretTSContext";
import { getETHCurrentLotteryId, getETHLotteryInfo } from "../../../../../common/integrations/contracts/EthJs/EthFunctions";
import { LotteryGameLoaderProps } from "../components/LotteryGames";

export default function ETHLotteryQuery(): LotteryGameLoaderProps {
    const [lotteryGameLoaderProps, setLotteryGameLoaderProps] =
        useState<LotteryGameLoaderProps>({
            currentLotteryID: 0
        });
    const { setCurrentLotteryID, setLotteryMapping } = useContext(LotteryContext);
    const { secretjs, Refresh } = useContext(SecretContext);
    var currentLotteryID = 0
    useEffect(() => {
        const fetchLottery = async () => { //TODO Fill in with all numbers
            currentLotteryID = await getETHCurrentLotteryId()
            console.log()
            setCurrentLotteryID(currentLotteryID)
            //setLCurrentLotteryID(id)
            var allLotteries: LotteryResponse[] = []
            for (var x = 1; x <= currentLotteryID; x++) {
                var lottery = await getETHLotteryInfo(x)
                console.log(lottery)
                allLotteries.push(lottery)
            }


            allLotteries.forEach((lottery, index) => {
                // Since your index starts at 1, add 1 to the array index
                setLotteryMapping(index + 1, lottery);
                console.log("Running again")
            });
            setLotteryGameLoaderProps({
                currentLotteryID: currentLotteryID
            })
        }
        fetchLottery();
    }, [Refresh]); //used to have predictionInfoAr,

    return lotteryGameLoaderProps;
}
