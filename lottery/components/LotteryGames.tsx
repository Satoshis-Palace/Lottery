import { LotteryResponse } from 'sp_lottery';
import CurLotteryBoard from './boards/CurLotteryBoard';
import LoadingPrediction from '../../predictions/components/prediction/PredictionLoading';
import LotteryQuery from './LotteryQuery';
import Card from "../../../../../common/card/Card"
import './LotteryGame.scss';
import PreviousLotteryBoards from './boards/PreviousLotteryBoards';

export function LotteryGame() {
  var lotteryqueryResponse = {}
  lotteryqueryResponse = LotteryQuery()

  return (<>
    {/* @ts-ignore */}
    {LotteryGameLoader(lotteryqueryResponse)}
  </>);
};

export interface LotteryGameLoaderProps {
  currentLotteryID: number
}
function LotteryGameLoader(props: LotteryGameLoaderProps) {
  if (props.currentLotteryID == 0) {
    return <>
      <LoadingPrediction></LoadingPrediction>
    </>
  }
  else {
    return (
      <div className='lotteryPage'>
        <Card width="95%" height="" className="lotteryCard">

          <div className="previousLotteryGame">
            <PreviousLotteryBoards />
          </div>
          <div className="currentLotteryGame">
            <CurLotteryBoard />
          </div>


        </Card>

      </div>

    )
  }
}

export default LotteryGame;
