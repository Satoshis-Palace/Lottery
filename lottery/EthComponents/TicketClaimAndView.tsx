import React, { useContext, useEffect, useState } from 'react';
import { LotteryContext } from '../LotteryContext';
import { SecretContext } from '../../../../../common/integrations/contracts/secretJs/SecretTSContext';
import { LotteryContract, Ticket } from 'sp_lottery';
import { getViewKey } from '../../../../../common/integrations/contracts/secretJs/SecretFunctions';
import ViewKeyButton from '../../../../../common/createViewKeyButton/CreateViewKeyButton';
import LotteryTicketViewer from './TicketDisplayer';
import { DEFAULT_TICKET } from '../../../../../common/integrations/contracts/Lottery';
import { getETHUserTickets } from '../../../../../common/integrations/contracts/EthJs/EthFunctions';

interface LotteryClaimAndViewProps {
  id: number
  max_height: string
}

function LotteryClaimAndView(props: LotteryClaimAndViewProps) {
  const { lotteryMapping, currentLotteryID, setUsersTicketsMapping } = useContext(LotteryContext);
  const { secretjs, Refresh } = useContext(SecretContext);
  const [tickets, setTickets] = useState<Ticket[]>([DEFAULT_TICKET]);
  var viewKey
  var TestWinningTicket: Ticket = props.id == currentLotteryID
    ? { numbers: lotteryMapping.get(currentLotteryID)?.numbers || [] }
    : { numbers: lotteryMapping.get(currentLotteryID - 1)?.numbers || [] };

  useEffect(() => {
    async function getUserTickets() {
      if (props.id == currentLotteryID) { //If this is for the current lottery
        var tix = await getETHUserTickets(currentLotteryID)
        if (tix.toString() != "Generic error: Generic error: unauthorized")
          setUsersTicketsMapping(currentLotteryID, tix)
      } else {
        var tix = await getETHUserTickets(currentLotteryID - 1)
        if (tix.toString() != "Generic error: Generic error: unauthorized")
          setUsersTicketsMapping(currentLotteryID - 1, tix)
      }
      var tix = await getETHUserTickets(props.id)
      console.log(tix)

      if (tix.toString() != "Generic error: Generic error: unauthorized") {
        setUsersTicketsMapping(props.id, tix)
        setTickets(tix)
      }
      console.log(tix)
    }
    getUserTickets()
  }, [Refresh]);

  return (<>
    {tickets.length != 0 ?
      <LotteryTicketViewer tickets={tickets} lottery_id={props.id} winning_tickets={TestWinningTicket} max_height={props.max_height} />
      :
      (props.id == currentLotteryID ? 'You currently have no Tickets. Feel free to test your luck and buy some tickets!' : 'You did not purchase any tickets for this lottery.')
    }
  </>);
};

export default LotteryClaimAndView;
