
export interface Ticker {
  name: string;
  fee: number;
  currentGoal: number;
  currentPercent: number;
}

export interface FinTableState {
  portfolio: Portfolio;
  columnsNames: Array<string>;
  columns: Array<string>;
}

export interface Portfolio {
  current_detail: PortfolioDetails;
  columnsNames: Array<string>;
  columns: Array<string>;
}

interface PortfolioDetails {
  stocks: Array<Ticker>;
  bonds: Array<Ticker>;
  goal_stock_percent: number;
  current_stock_percent: number;
  deviation_percent: number;
}
