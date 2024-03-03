import { useContext, useCallback } from 'react';
import "./TicketPurchaser.scss"
import { LotteryContext } from '../LotteryContext';
import { LotteryContract, Ticket } from 'sp_lottery';
import getLotteryContract from '../../../../../common/integrations/contracts/Lottery';
import { SecretContext } from '../../../../../common/integrations/contracts/secretJs/SecretTSContext';
import { getViewKey } from '../../../../../common/integrations/contracts/secretJs/SecretFunctions';
import { launchLotteryMessageToast } from '../../toast/LaunchToast';

function BuySection() {
    const { currentLotteryID, lotteryMapping, usersNumbers } = useContext(LotteryContext);
    const { secretjs, Refresh } = useContext(SecretContext);
    const lottery_contract = getLotteryContract(secretjs);
    const cost = lotteryMapping.get(currentLotteryID)?.cost || 0;

    const buyTickets = useCallback(async (numbersAR: string[][]) => {
        const ticketAR = numbersAR.map(numbers => ({ numbers } as Ticket));
        try {
            const prom = lottery_contract.buyTickets(ticketAR, secretjs.address);
            await launchLotteryMessageToast(prom, "suc", "Failed to buy tickets.");
            const viewKeyy = await getViewKey(false, true, secretjs, "Lottery");
            await Refresh();
        } catch (error) {
            console.error("Error buying tickets:", error);
            // Handle error
        }
    }, [lottery_contract, secretjs, Refresh]);

    return (
        <button className="buyButton" onClick={() => buyTickets(usersNumbers)}> BUY! (${usersNumbers.length * cost})</button>
    );
}

export default BuySection;
