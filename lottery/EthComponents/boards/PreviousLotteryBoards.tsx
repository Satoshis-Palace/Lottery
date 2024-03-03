import React, { useContext, useEffect } from 'react';
import FilledSpacer from '../../../../../../common/card/FilledSpacer';
import PastLotteryGrid from '../PastLotteryGrid';
import PastLotteries from '../PastLotteries';
import './AllLotteryBoard.scss';

export const PreviousLotteryBoards: React.FC = () => {
  return (
    <>
      <div className='lotteryBanner'>Previous Lottery</div>
      <FilledSpacer width={'100%'} height={'0.35rem'} />
      <PastLotteryGrid />
      <PastLotteries />
    </>
  );
};

export default PreviousLotteryBoards;
