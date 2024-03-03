import React, { useContext, useState, useEffect } from 'react';
import { LotteryContext } from '../LotteryContext';
import { LotteryContract } from 'sp_lottery';
import { SecretContext } from '../../../../../common/integrations/contracts/secretJs/SecretTSContext';
import { Icon } from '@iconify/react';
import "./TicketStager.scss"
import FilledSpacer from '../../../../../common/card/FilledSpacer';

const TicketStager: React.FC = () => {
  const { currentLotteryID, lotteryMapping, setUsersNumbers } = useContext(LotteryContext);
  const { secretjs } = useContext(SecretContext);
  const [numberOfTicketsToBuy, setNumberOfTicketsToBuy] = useState('1');
  const [numbers, setNumbers] = useState<string[][]>([]);
  const length = lotteryMapping.get(currentLotteryID)?.difficulty_num || 0;

  // Initialize numbers and userNumbers states when component mounts
  useEffect(() => {
    const initialNumbers = Array.from({ length: parseInt(numberOfTicketsToBuy) }, () =>
      Array.from({ length: length }, () =>
        Math.floor(Math.random() * 100).toString()));
    setNumbers(initialNumbers);
    setUsersNumbers(initialNumbers);
  }, [numberOfTicketsToBuy, length, setUsersNumbers]);

  const handleTicketNumbersChange = (row: number, col: number) => (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    const newNumbers = [...numbers];
    let value = event.target.value;

    if (value === '' || (!isNaN(parseInt(value)) && !isNaN(parseFloat(value)))) {
      newNumbers[row][col] = value;
      setNumbers(newNumbers);
      setUsersNumbers(newNumbers);
    } else {
      newNumbers[row][col] = '';
      setNumbers(newNumbers);
      setUsersNumbers(newNumbers);
    };
  };

  const handleTotalTicketChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;

    if (value === '' || (!isNaN(parseInt(value)) && parseInt(value) >= 0 && parseInt(value) <= 99)) {
      setNumberOfTicketsToBuy(value);
    }
  };

  const randomizeNumbers = () => {
    const randomizedNumbers = Array.from({ length: parseInt(numberOfTicketsToBuy) }, () =>
      Array.from({ length: length }, () =>
        Math.floor(Math.random() * 100).toString()));
    setNumbers(randomizedNumbers);
    setUsersNumbers(randomizedNumbers); // Update user numbers after randomizing
  };

  const individualRandomizeNumbers = (rowIndex: number) => {
    const randomizedRow = Array.from({ length: length }, () => Math.floor(Math.random() * 100).toString());
    setNumbers(currentNumbers => {
      const updatedNumbers = currentNumbers.map((row, index) =>
        index === rowIndex ? randomizedRow : row);
      setUsersNumbers(updatedNumbers); // Update user numbers after randomizing a row
      return updatedNumbers; // Return updated numbers for setting state
    });
  };

  // const pullLottery = () => {
  //   var prom = lottery_contract.pullLottery()
  //   console.log(prom)
  // };

  // const pullLotteryAdmin = () => {
  //   var prom = lottery_contract.pullLotteryAdmin('3', '10', '25000000000000000000') //difficulty_num: string, length: string, cost: string
  //   console.log(prom)
  // };

  return (
    <div className="NumberSelectorContainer">
      <div className="buttonsContainer">
        <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
          {/* # of Tickets:
          <input
            type="text"
            className="TicketAmountEntry"
            value={numberOfTicketsToBuy}
            onChange={handleTotalTicketChange}
            placeholder="Tickets"
            maxLength={2}
          /> */}
        </div>
        <div className='curLotteryBuy' style={{ fontSize: '1.5rem', cursor: 'pointer' }} onClick={() => randomizeNumbers()}>
          Randomize
          <Icon style={{ color: 'black', fontSize: '3rem' }} className="individualRandomizeButton"
            icon={"material-symbols:sync"}
          />
        </div>
      </div>
      <FilledSpacer width={'100%'} height={'0.35rem'} />
      <div className="NumberSelectorHolder">
        {numbers.map((row, rowIndex) => (
          <div key={rowIndex} className="numberRow">
            <Icon className="individualRandomizeButton"
              icon={"material-symbols:refresh-rounded"}
              onClick={() => individualRandomizeNumbers(rowIndex)} />
            {row.map((num, colIndex) => (
              <input
                key={colIndex}
                type="text"
                pattern="\d*"
                className="numberInput"
                value={num}
                onChange={handleTicketNumbersChange(rowIndex, colIndex)}
                placeholder="0-99"
              />
            ))}
            <div style={{ color: 'white' }}>
              Ticket {rowIndex + 1} of {numberOfTicketsToBuy}
            </div>
          </div>
        ))}
      </div>
    </div>

  );
};

export default TicketStager;
