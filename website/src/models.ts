
export class Ticker {
  name: string;
  fee: integer;
  currentGoal: integer;
  currentPercent: integer;
}

export interface FinTableState {
  tickerList: Array<Ticker>;
}
