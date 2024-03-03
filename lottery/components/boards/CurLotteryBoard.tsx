import React, { useContext, useEffect } from 'react';
import { LotteryContext } from '../../LotteryContext';
import TicketStager from '../TicketStager';
import LotteryPotSize from '../PotSize';
import LotteryClaimAndView from '../TicketClaimAndView';
import LotteryDetails from '../LotteryDetails';
import FilledSpacer from '../../../../../../common/card/FilledSpacer';
import BuySection from '../BuySection';
import './AllLotteryBoard.scss';

export const CurLotteryBoard: React.FC = () => {
  const { currentLotteryID } = useContext(LotteryContext);
  const gridStyle: React.CSSProperties = {
    display: 'flex',
    gridTemplateColumns: 'repeat(2, 1fr)',
    gridTemplateRows: 'auto',
    width: "100%",
    color: "white",
    overflow: 'hidden',
  };
  const cellStyle: React.CSSProperties = {
    width: "100%",
    height: "100%",
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
  };

  return (
    <>
      <div className='lotteryBanner'>
        <div style={{ marginRight: '2rem', fontSize: '1.5rem' }}>
          #{currentLotteryID}
        </div>
        Current Lottery
      </div>
      <FilledSpacer width={'100%'} height={'0.35rem'} />
      <div className='contentContainer' style={gridStyle}>
        <div style={cellStyle}>
          <TicketStager />
        </div>
        <div style={cellStyle} className="infoAndBuy">
          <div className='curLotteryInfo'>
            <LotteryPotSize id={currentLotteryID} />
            <LotteryDetails id={currentLotteryID} />
          </div>
          <div style={{ marginBottom: '1rem' }}>
            <BuySection />
          </div>
          <div className='curLotteryTickets'>
            <LotteryClaimAndView id={currentLotteryID} max_height={'14rem'} />
          </div>
        </div>
      </div>
    </>
  );
};

export default CurLotteryBoard;
