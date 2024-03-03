
import { getViewKey } from "../../../../../common/integrations/contracts/secretJs/SecretFunctions";
import './LotteryNumbers.scss';
import { useContext, useEffect, useState } from "react";
import getLotteryContract from "../../../../../common/integrations/contracts/Lottery";
import { LotteryContext } from "../LotteryContext";
import { SecretContext } from "../../../../../common/integrations/contracts/secretJs/SecretTSContext";
import { LotteryContract } from "sp_lottery";
interface LotteryNumbersProps {
  numbers: string[];
  id: number;
}

const LotteryNumbers: React.FC<LotteryNumbersProps> = (props: LotteryNumbersProps) => {
  const { secretjs, Refresh } = useContext(SecretContext);
  var viewKey
  const [startTime, setStartTime] = useState<number>(0);
  const [endTime, setEndTime] = useState<number>(0);
  const { lotteryMapping } = useContext(LotteryContext);

  useEffect(() => {
    const fetchLottery = async () => {
      viewKey = await getViewKey(false, false, secretjs, "Lottery") || "No KEY"

      setStartTime(lotteryMapping.get(props.id)?.start_time || 0)
      setEndTime(lotteryMapping.get(props.id)?.end_time || 0)
    }
    fetchLottery();
  }, [Refresh]);

  function convertUnixTimeToDate(unixTime: number): string {
    const date = new Date(unixTime * 1000); // Convert to milliseconds
    return date.toLocaleString(); // Convert to local date string
  }
  return (
    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
      <div className="lotteryNumberContainer">
        <div className="lotteryNumberBanner">Pulled Numbers:</div>
        <div style={{ display: 'flex', flexDirection: 'row', justifyContent: 'space-around', width: '90%' }}>
          {props.numbers.map((number, index) => (
            <div className="lottery-number" key={index}>
              {number}
            </div>
          ))}
        </div>

      </div>
      <div style={{ display: 'flex', flexDirection: 'row' }}>
        <label style={{ fontSize: ".75rem", display: "flex", textAlign: "center" }}> Start date: {convertUnixTimeToDate(startTime)} </label>
        <label style={{ fontSize: ".75rem", display: "flex", textAlign: "center" }}>End date: {endTime != 0 ? convertUnixTimeToDate(endTime) : "Lottery has not ended"} </label>
      </div>

    </div>

  );
};

export default LotteryNumbers;
