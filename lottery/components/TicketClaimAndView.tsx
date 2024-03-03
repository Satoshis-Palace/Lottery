import React, { useContext, useEffect, useState } from 'react';
import { LotteryContext } from '../LotteryContext';
import { SecretContext } from '../../../../../common/integrations/contracts/secretJs/SecretTSContext';
import { LotteryContract, Ticket } from 'sp_lottery';
import getLotteryContract, { DEFAULT_TICKET } from '../../../../../common/integrations/contracts/Lottery';
import { getViewKey } from '../../../../../common/integrations/contracts/secretJs/SecretFunctions';
import ViewKeyButton from '../../../../../common/createViewKeyButton/CreateViewKeyButton';
import LotteryTicketViewer from './TicketDisplayer';

interface LotteryClaimAndViewProps {
  id: number
  max_height: string
}

function LotteryClaimAndView(props: LotteryClaimAndViewProps) {
  const { lotteryMapping, currentLotteryID, setUsersTicketsMapping } = useContext(LotteryContext);
  const { secretjs, Refresh } = useContext(SecretContext);
  const [tickets, setTickets] = useState<Ticket[]>([DEFAULT_TICKET]);
  var lottery_contract: LotteryContract = getLotteryContract(secretjs)
  var viewKey
  var TestWinningTicket: Ticket = props.id == currentLotteryID
    ? { numbers: lotteryMapping.get(currentLotteryID)?.numbers || [] }
    : { numbers: lotteryMapping.get(currentLotteryID - 1)?.numbers || [] };

  useEffect(() => {
    async function getUserTickets() {
      viewKey = await getViewKey(false, false, secretjs, "Lottery") || "NO KEY"
      if (viewKey != "NO KEY") { // Only fetch stuff if key exists
        if (props.id == currentLotteryID) { //If this is for the current lottery
          var tix = await lottery_contract.getUsersTickets(secretjs.address, currentLotteryID, viewKey)
          if (tix.toString() != "Generic error: Generic error: unauthorized")
            setUsersTicketsMapping(currentLotteryID, tix)
        } else {
          var tix = await lottery_contract.getUsersTickets(secretjs.address, currentLotteryID - 1, viewKey)
          if (tix.toString() != "Generic error: Generic error: unauthorized")
            setUsersTicketsMapping(currentLotteryID - 1, tix)
        }
        var tix = await lottery_contract.getUsersTickets(secretjs.address, props.id, viewKey)
        if (tix.toString() != "Generic error: Generic error: unauthorized") {
          setUsersTicketsMapping(props.id, tix)
          setTickets(tix)
        }
      }
    }
    getUserTickets()
  }, [Refresh]);

  return (<>
    {tickets[0] == DEFAULT_TICKET ?
      <ViewKeyButton type={"Lottery"} /> :
      tickets.length != 0 ?
        <LotteryTicketViewer tickets={tickets} lottery_id={props.id} winning_tickets={TestWinningTicket} lottery_contract={lottery_contract} max_height={props.max_height} />
        :
        (props.id == currentLotteryID ? 'You currently have no Tickets. Feel free to test your luck and buy some tickets!' : 'You did not purchase any tickets for this lottery.')
    }
  </>);
};

export default LotteryClaimAndView;
