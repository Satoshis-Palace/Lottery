import { useContext, useState } from 'react';
import { LotteryContext } from '../LotteryContext';
import { LotteryContract, Ticket } from 'sp_lottery';
import { SecretContext } from '../../../../../common/integrations/contracts/secretJs/SecretTSContext';
import { getViewKey } from '../../../../../common/integrations/contracts/secretJs/SecretFunctions';
import { launchLotteryMessageToast } from '../../toast/LaunchToast';
import RoundedButton from '../../../../../common/roudButton/RoundedButton';
import { lineWobble } from 'ldrs'
import './boards/AllLotteryBoard.scss';
import { sats2Tokens } from '../../../../../common/integrations/contracts/snip20';
import { SubmitGoal } from '../../../../../Tracking/EventGoals';
import { buyEthLotteryTicket } from '../../../../../common/integrations/contracts/EthJs/EthFunctions';

lineWobble.register()
function BuySection() {
    const { currentLotteryID, lotteryMapping, usersNumbers, isLoading, changeLoadingStatus } = useContext(LotteryContext);
    const { secretjs, Refresh } = useContext(SecretContext);
    var cost = parseFloat((lotteryMapping.get(currentLotteryID)?.cost || 0).toString())
    async function buyTickets(numbersAR: string[][]) { //TODO check for tickets you already own
        changeLoadingStatus(true)
        var ticketAR = []
        for (var index = 0; index < numbersAR.length; index++) {
            ticketAR.push({ numbers: numbersAR[index] } as Ticket)
        }
        var prom = buyEthLotteryTicket(numbersAR[0])
        //await launchLotteryMessageToast(prom, "Successfully bought " + ticketAR.length + " tickets for" + (cost * ticketAR.length) + " USDC", "Failed to buy tickets. ")
        //var viewKeyy = await getViewKey(false, true, secretjs, "Lottery")
        await Refresh()
        SubmitGoal('LotteryBet')
        changeLoadingStatus(false)
    };

    return (
        <div className='curLotteryBuy'>
            <div>
                {/* <div>Total Tickets : {usersNumbers.length} </div> */}
                <div>Tickets Price : {cost} wei</div>
                {/* <div>Total Price : {usersNumbers.length * cost} USDC</div> */}
            </div>

            <RoundedButton className="buyButton" onClick={() => buyTickets(usersNumbers)} width={'5.5rem'} height={'2.5rem'}> {isLoading ? <l-line-wobble
                size="50"
                stroke="7"
                bg-opacity="0.25"
                speed="3"
                color="white"
            ></l-line-wobble> : 'Play'}</RoundedButton>
        </div>

    );
};

export default BuySection;