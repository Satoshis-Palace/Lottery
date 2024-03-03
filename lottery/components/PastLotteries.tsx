import React, { useContext } from 'react';
import LotteryNumbers from './LotteryNumbers';
import LotteryPotSize from './PotSize';
import { LotteryContext } from '../LotteryContext';
import FilledSpacer from '../../../../../common/card/FilledSpacer';

const PastLotteries: React.FC = () => {
    const gridStyle: React.CSSProperties = {
        display: 'grid',
        gridTemplateColumns: '6fr 4fr',
        gridTemplateRows: 'repeat(1, 8rem)',
        width: "100%",
        color: "white"
    };
    const cellStyle: React.CSSProperties = {
        width: "100%",
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        textAlign: 'center',
        justifyContent: 'center'
    };
    const { lotteryMapping, currentLotteryID } = useContext(LotteryContext);
    const previousLotteryIDs = Array.from({ length: currentLotteryID - 2 }, (_, i) => currentLotteryID - 2 - i);

    return (
        <>
            <div className='previousLotteriesContainer'>
                {previousLotteryIDs.map(id => (
                    <React.Fragment key={id}>
                        <div style={gridStyle}>
                            <div style={{ ...cellStyle, position: 'relative' }}>
                                <div style={{ position: 'absolute', top: '0', left: '0', backgroundColor: 'black', borderRadius: '.5rem', padding: '.1rem' }}>
                                    #{id}
                                </div>
                                <div style={{ display: 'flex', justifyContent: 'center' }}>
                                    <LotteryNumbers numbers={lotteryMapping.get(id)?.numbers || []} id={id} />
                                </div>
                            </div>
                            <div style={{ ...cellStyle, textAlign: 'center' }}>
                                <LotteryPotSize id={id} />
                            </div>
                        </div>
                        <FilledSpacer width={'100%'} height={'0.35rem'} />
                    </React.Fragment>
                ))}
            </div>
        </>
    );
};

export default PastLotteries;
