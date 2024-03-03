import { useContext, useEffect, useState } from "react";
import getLotteryContract from "../../../../../common/integrations/contracts/Lottery";
import { LotteryContext } from "../LotteryContext";
import { SecretContext } from "../../../../../common/integrations/contracts/secretJs/SecretTSContext";
import { LotteryContract } from "sp_lottery";
import ViewKeyButton from "../../../../../common/createViewKeyButton/CreateViewKeyButton";
import { getViewKey } from "../../../../../common/integrations/contracts/secretJs/SecretFunctions";
import { Icon } from "@iconify/react";
import './LotteryDetails.scss'
import { sats2Tokens } from "../../../../../common/integrations/contracts/snip20";

interface LotteryDetailsProps {
  id: number
}

function LotteryDetails(props: LotteryDetailsProps) {
  const { currentLotteryID, lotteryMapping } = useContext(LotteryContext);
  const { secretjs, Refresh } = useContext(SecretContext);
  var viewKey
  const [startTime, setStartTime] = useState<number>(0);
  const [endTime, setEndTime] = useState<number>(0);
  const [totalMoney, setTotalMoney] = useState<number>(0);
  const [totalTickets, setTotalTickets] = useState<number>(0);
  const [difficulty, setDifficulty] = useState<number>(0);
  const previousLotteryID = currentLotteryID - 1;
  var lottery_contract: LotteryContract = getLotteryContract(secretjs)

  useEffect(() => {
    const fetchLottery = async () => {
      viewKey = await getViewKey(false, false, secretjs, "Lottery") || "NO KEY"
      if (viewKey != "NO KEY") { // Only fetch stuff if key exists
        // var holder = await lottery_contract.getUsersTotalTickets(secretjs.address, viewKey)
        // setTotalUserTickets(parseInt(holder))
        // var holder = await lottery_contract.getTotalMoneyCollected()
        // setTotalMoney(parseInt(holder))
      }
      //TOdo maybe better way to get this
      setStartTime(lotteryMapping.get(props.id)?.start_time || 0)
      setEndTime(lotteryMapping.get(props.id)?.end_time || 0)
      setDifficulty(lotteryMapping.get(props.id)?.difficulty_num || 0)
      if (props.id == currentLotteryID) {
      var curLotteryInfo = await lottery_contract.getCurrentLotteryInfo()
      //@ts-ignore
      setTotalTickets(curLotteryInfo.tickets_sold || 0)
      setTotalMoney(parseFloat(sats2Tokens((lotteryMapping.get(props.id)?.amount || 0).toString())))
      }
      else {
        //@ts-ignore
        setTotalTickets(lotteryMapping.get(props.id)?.tickets_sold || 0)
        setTotalMoney(parseFloat(sats2Tokens((lotteryMapping.get(props.id)?.amount || 0).toString())))
      }
    }
    fetchLottery();
  }, [Refresh]); //used to have predictionInfoAr,

  function convertUnixTimeToDate(unixTime: number): string {
    const date = new Date(unixTime * 1000); // Convert to milliseconds
    return date.toLocaleString(); // Convert to local date string
  }

  return (
    <>
      {totalMoney == 0 ? "Loading..." :
        <div className="LotteryDetails">
          {props.id == currentLotteryID ? <>
            <label style={{ fontSize: "1rem", opacity: '.8' }}> Start date: {convertUnixTimeToDate(startTime)} </label>
            <label style={{ fontSize: "1rem", opacity: '.8' }}>End date: {endTime != 0 ? convertUnixTimeToDate(endTime) : "Lottery has not ended"} </label>
          </> : ''}
          {props.id == currentLotteryID || previousLotteryID ? <>
            <div style={{
              display: "flex",
              flexDirection: "column"
            }} >
              <div style={{ display: "flex", alignItems: "left", justifyContent: 'left' }}>
                <Icon className="lotteryIcon" icon={"game-icons:strongbox"} />
                <span style={{ fontSize: "1.25rem" }}>Difficulty: {difficulty}</span>
              </div>
              <div style={{ display: "flex", alignItems: "left", justifyContent: 'left' }}>
                <Icon className="lotteryIcon" icon={"icon-park-outline:ticket"} />
                {/* TODO: totalUserTickets functionality should change to total tickets purchased per our design. */}
                <span style={{ fontSize: "1.25rem" }}>Tickets Sold: {totalTickets}</span>
              </div>
              <div style={{ display: "flex", alignItems: "left", justifyContent: 'left' }}>
                <Icon className="lotteryIcon" icon={"icon-park-solid:chart-line-area"} />
                <span style={{ fontSize: "1.25rem" }}>Pot Raised: {totalMoney.toFixed(2)} USDC</span>
              </div>
              <div style={{ display: "flex", alignItems: "center", justifyContent: 'center' }}>
                {/* <Icon className="lotteryIcon" icon={"material-symbols:person"} /> */}
                {/* TODO: no way to currently see how many unique users purchased tickets. going to display totalMoney/10 for now */}
                {/* <span style={{ fontSize: "1.25rem" }}>Unique Users: {totalMoney / 10}</span> */}
              </div>
            </div>
          </> : ""}
        </div>
      }
    </>
  );
};

export default LotteryDetails;
