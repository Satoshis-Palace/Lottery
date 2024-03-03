import React, { createContext, useState, ReactNode } from "react";
import { LotteryResponse, Ticket } from "sp_lottery";
import { DEFAULT_LOTTERY, DEFAULT_TICKET } from "../../../../common/integrations/contracts/Lottery";

interface LotteryContextType {
  usersNumbers: string[][],
  setUsersNumbers: (active: string[][]) => void,
  currentLotteryID: number,
  setCurrentLotteryID: (active: number) => void,
  lotteryMapping: Map<number, LotteryResponse>;
  setLotteryMapping: (id: number, lottery: LotteryResponse) => void;
  usersTicketsMapping: Map<number, Ticket[]>;
  setUsersTicketsMapping: (id: number, tickets: Ticket[]) => void;
  isLoading: (boolean);
  setIsLoading: (status: boolean) => void;
  changeLoadingStatus: (status: boolean) => void;
  totalTicketsSold: number,
  setTotalTicketsSold: (active: number) => void,
  uniqueUsers: number,
  setUniqueUsers: (active: number) => void,
}

export const LotteryContext = createContext<LotteryContextType>({
  usersNumbers: [[]],
  setUsersNumbers: () => { },
  currentLotteryID: 0,
  setCurrentLotteryID: () => { },
  lotteryMapping: new Map<number, LotteryResponse>,
  setLotteryMapping: () => { },
  usersTicketsMapping: new Map<number, Ticket[]>,
  setUsersTicketsMapping: () => { },
  isLoading: false,
  setIsLoading: () => { },
  changeLoadingStatus: () => { },
  totalTicketsSold: 0,
  setTotalTicketsSold: () => { },
  uniqueUsers: 0,
  setUniqueUsers: () => { },
});
interface LotteryProviderProps {
  children: ReactNode;
}

export const LotteryProvider: React.FC<LotteryProviderProps> = ({ children }) => {
  const [usersNumbers, setUsersNumbers] = useState<string[][]>([[]])
  const [currentLotteryID, setCurrentLotteryID] = useState<number>(0)
  const [lotteryMapping] = useState<Map<number, LotteryResponse>>(new Map<number, LotteryResponse>);
  const [usersTicketsMapping] = useState<Map<number, Ticket[]>>(new Map<number, Ticket[]>);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [totalTicketsSold, setTotalTicketsSold] = useState<number>(0);
  const [uniqueUsers, setUniqueUsers] = useState<number>(0);

  lotteryMapping.set(0, DEFAULT_LOTTERY)
  usersTicketsMapping.set(0, [DEFAULT_TICKET])

  async function setLotteryMapping(id: number, lottery: LotteryResponse) {
    lotteryMapping.set(id, lottery)
  }
  async function setUsersTicketsMapping(id: number, tickets: Ticket[]) {
    usersTicketsMapping.set(id, tickets)
  }

  async function changeLoadingStatus(status: boolean) {
    setIsLoading(status)
  }
  return (
    <LotteryContext.Provider
      value={{
        usersNumbers,
        setUsersNumbers,
        currentLotteryID,
        setCurrentLotteryID,
        lotteryMapping,
        setLotteryMapping,
        usersTicketsMapping,
        setUsersTicketsMapping,
        isLoading,
        setIsLoading,
        changeLoadingStatus,
        totalTicketsSold,
        setTotalTicketsSold,
        uniqueUsers,
        setUniqueUsers
      }}
    >
      {children}
    </LotteryContext.Provider>
  );
};