import { Action, Ticker } from "../../data/models"

export interface BuyNextData {
  goal_id: number,
  port_a_id: number,
  actions: Action[]
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

export interface BuyNextResp {
  actions: Array<Action>;
  buy_value: number;
  action_summary: Map<number, string>;
}
