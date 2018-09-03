
export class Ticker {
  name: string;
  fee: integer;
  currentGoal: integer;
  currentPercent: integer;
}

export interface FinTableState {
  tickerList: [Ticker];
  msg: string;
}
