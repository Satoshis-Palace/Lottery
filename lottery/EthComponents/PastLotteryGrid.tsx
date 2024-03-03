import React, { useContext } from 'react';
import LotteryNumbers from './LotteryNumbers';
import { LotteryContext } from '../LotteryContext';
import LotteryClaimAndView from './TicketClaimAndView';
import LotteryPotSize from './PotSize';
import LotteryDetails from './LotteryDetails';
import FilledSpacer from '../../../../../common/card/FilledSpacer';

const PastLotteryGrid: React.FC = () => {
    const gridStyle: React.CSSProperties = {
        display: 'grid',
        gridTemplateColumns: '5fr 5fr',
        gridTemplateRows: 'repeat(2, 8rem)',
        width: "100%",
        color: "white",
    };
    const cellStyle: React.CSSProperties = {
        width: "100%",
        display: 'flex',
        textAlign: 'left',
        height: "10rem",
    };
    const { lotteryMapping, currentLotteryID } = useContext(LotteryContext);
    const previousLotteryID = currentLotteryID - 1;

    return (
        <>
            <div style={gridStyle} key={previousLotteryID} className='LastLotteryData'>
                <div style={{ ...cellStyle, justifyContent: 'end', width: '100%', position: 'relative' }}>
                    <div style={{ position: 'absolute', top: '0', left: '0', backgroundColor: 'black', borderRadius: '.5rem', padding: '.1rem' }}>
                        #{previousLotteryID}
                    </div>
                    <div style={{ display: 'flex', justifyContent: 'center' }}>
                        <LotteryNumbers numbers={lotteryMapping.get(previousLotteryID)?.numbers || []} id={previousLotteryID} />
                    </div>
                </div>
                <div style={{ ...cellStyle, justifyContent: 'center' }}>
                    <LotteryPotSize id={previousLotteryID} />
                </div>
                <div style={{
                    display: "flex",
                    flexDirection: "row",
                    alignItems: 'center',
                    justifyContent: 'end'
                }} >
                    <LotteryDetails id={previousLotteryID} />
                </div>
                <div style={{ ...cellStyle, justifyContent: 'center' }}>
                    <LotteryClaimAndView id={previousLotteryID} max_height={'6.5rem'} />
                </div >
            </div >
            <FilledSpacer width={'100%'} height={'0.35rem'} />
        </>

    );
};

export default PastLotteryGrid;