
export interface Ticker {
  name: string;
  fee: integer;
  currentGoal: integer;
  currentPercent: integer;
}

export interface FinTableState {
  msg: string;
  tickerList: [Ticker];
}
