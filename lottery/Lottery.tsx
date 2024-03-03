import { Modal } from '../../../../common/modal/Modal';
import { ETHLotteryGame } from './EthComponents/LotteryGames';
import LotteryGame from "./components/LotteryGames"
import "./lottery.scss"
import { useState } from "react";

export default function Lottery() {
    const [isModalVisible, setIsModalVisible] = useState(true);
    const [showETHLottery, setShowETHLottery] = useState(false); // false shows LotteryGame by default

    return (
        <>
            <div style={{ padding: '10px' }}>
                <button onClick={() => setShowETHLottery(!showETHLottery)}>
                    Switch to {showETHLottery ? 'LotteryGame' : 'ETHLotteryGame'}
                </button>
            </div>
            <div>
                {isModalVisible ? (
                    <Modal
                        title="Welcome to Lottery Development!"
                        description="This page is currently under development :)"
                        onClose={() => setIsModalVisible(false)}
                    />
                ) : (
                    <>
                        {showETHLottery ? <ETHLotteryGame /> : <LotteryGame />}
                    </>
                )}
            </div>
        </>
    );
}