
export interface Ticker {
  symbol: string;
  fee: number;
  currentGoal: number;
  currentPercent: number;
  goal_percent: number;
  actual_percent: number;
}

export interface FinTableState {
  portfolio: Portfolio;
  columnsNames: Array<string>;
  columns: Array<string>;
}

interface Portfolio {
  stocks: Array<Ticker>;
  bonds: Array<Ticker>;
  goal_stock_percent: number;
  current_stock_percent: number;
  deviation_percent: number;
}

//======== buy next
export interface Action {
  id: number;
  shares: number;
  price: number;
}

//======== portfolio list
export interface PortfolioGoalList {
  id: number;
  name: string;
  goal_stock_percent: number;
  deviation_percent: number;
  description: string;
}
