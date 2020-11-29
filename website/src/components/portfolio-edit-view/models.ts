export interface EditData {
  goalPortId: number;
  stockPercent: number;
  name: string;
  description: string;
}
export interface FinPortfolioResp {
  name: string,
  goal_id: number;
  tickers: Array<Ticker>;
  goal_stock_percent: number;
  actual_stock_percent: number;
  total_value: number;
  deviation_percent: number;
}
