import { useContext, useEffect, useState } from 'react';
import './LotteryPot.scss';
import { LotteryContext } from '../LotteryContext';
import { SecretContext } from '../../../../../common/integrations/contracts/secretJs/SecretTSContext';
import { LotteryContract } from 'sp_lottery';
import getLotteryContract from '../../../../../common/integrations/contracts/Lottery';
import { bigMath, sats2Tokens } from '../../../../../common/integrations/contracts/snip20';

interface LotteryPotProps {
  id: number
}
function LotteryPotSize(props: LotteryPotProps) {
  const { lotteryMapping, currentLotteryID, setLotteryMapping } = useContext(LotteryContext);
  const { secretjs, Refresh } = useContext(SecretContext);
  const [potSize, setPotSize] = useState<number>(0)
  const [wasWinningTicketBought, setWasWinningTicketBought] = useState<boolean>(false)
  var lottery_contract: LotteryContract = getLotteryContract(secretjs)
  var winningTicket
  useEffect(() => {
    const fetchLottery = async () => {
      winningTicket = lotteryMapping.get(currentLotteryID - 1)?.numbers || ['']
      var tempWinnerholder = await lottery_contract.getTicketsUsers({ numbers: winningTicket }, currentLotteryID - 1)
      setWasWinningTicketBought(parseInt(await lottery_contract.getTicketsUsers({ numbers: winningTicket }, currentLotteryID - 1)) > 0)
      if (props.id == currentLotteryID) {
        var curLotteryInfo = await lottery_contract.getCurrentLotteryInfo()
        setLotteryMapping(currentLotteryID, curLotteryInfo)
        var lastLotteryPot = lotteryMapping.get(currentLotteryID - 1)?.amount.toString() || '0'
        var currLotteryPot = lotteryMapping.get(currentLotteryID)?.amount.toString() || '0'
        tempWinnerholder ? setPotSize(parseFloat(sats2Tokens(currLotteryPot))) : setPotSize(parseFloat(sats2Tokens((bigMath(lastLotteryPot, currLotteryPot)))));
      } else {
        setPotSize(parseFloat(sats2Tokens((lotteryMapping.get(props.id)?.amount.toString() || '0'))))
      }
    }
    fetchLottery();
  }, [Refresh]); //Refreshes this prop whern called
  return (
    <div className="lotteryPotDisplayContent">
      {(props.id === currentLotteryID || props.id === (currentLotteryID - 1)) ?
        <>
          <span style={{ fontSize: '1.5rem', color: 'white' }}>Pot Size:</span>
          <span style={{ fontSize: '2rem', color: '#13ED00' }}>{potSize.toFixed(3)} USDC</span>
          {props.id == (currentLotteryID - 1) ?
            <>
              <span style={{ fontSize: '1rem' }}>Claim Status: </span>
              <span style={{ fontSize: '.75rem', opacity: .8, display: 'flex', textAlign: 'center' }}>
                {props.id != currentLotteryID ? (wasWinningTicketBought ? "Winning ticket was bought. Winner may claim" : "Winning ticket was not bought. This pot will rollover to next week") : ""}
              </span>
            </>
            :
            ''}
        </>
        :
        <span style={{ fontSize: '1.5rem', color: 'red' }}>No Winning Ticket Claimed</span>
      }
    </div>
  );
};

export default LotteryPotSize;
