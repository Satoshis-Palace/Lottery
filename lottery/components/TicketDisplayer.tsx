
import { LotteryContract, Ticket } from 'sp_lottery';
import './TicketDisplayer.scss';
import { launchLotteryMessageToast } from '../../toast/LaunchToast';
import { useContext } from 'react';
import { SecretContext } from '../../../../../common/integrations/contracts/secretJs/SecretTSContext';

interface LotteryTicketViewerProps {
  tickets: Ticket[];
  lottery_id: number;
  winning_tickets: Ticket;
  lottery_contract: LotteryContract;
  max_height: string
}

function LotteryTicketViewer(props: LotteryTicketViewerProps) {
  const ticketArray = Object.values(props.tickets);
  return (
    <>
      <span style={{ fontSize: '.75rem' }}>Your Tickets:</span>
      <div className="viewer" style={{ maxHeight: props.max_height }}>
        {ticketArray.map((ticket, index) => (
          <LotteryTicket key={index} ticket={ticket} lottery_id={props.lottery_id} winning_ticket={props.winning_tickets} lottery_contract={props.lottery_contract} />
        ))}
      </div>
    </>

  );
};

export default LotteryTicketViewer;

interface LotteryTicketProps {
  ticket: Ticket;
  lottery_id: number;
  winning_ticket: Ticket;
  lottery_contract: LotteryContract
}

function LotteryTicket(props: LotteryTicketProps) {
  const { secretjs, Refresh } = useContext(SecretContext);

  async function handleTicketClick() {
    if (isWinningTicket()) {
      var prom = props.lottery_contract.ClaimTicket(props.ticket)
      await launchLotteryMessageToast(prom, "Successfully claimed lottery ticket", "Failed to claim tickets. ")
      await Refresh()
    }
  };

  const isWinningTicket = () => {
    if (props.ticket.numbers.length === undefined || (props.ticket.numbers.length !== props.winning_ticket.numbers.length)) {
      return false;
    }
    for (let i = 0; i < props.ticket.numbers.length; i++) {
      if (props.ticket.numbers[i] !== props.winning_ticket.numbers[i]) {
        return false;
      }
    }
    return true;
  };

  return (
    <div className={`ticket ${isWinningTicket() ? "winner" : ""}`} onClick={handleTicketClick}>
      {props.ticket.numbers.map((number, index) => (
        <div key={index} className="number-box">{number}</div>
      ))}
    </div>
  );
};