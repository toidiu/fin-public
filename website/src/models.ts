
export interface Ticker {
  name: string;
  fee: number;
  currentGoal: number;
  currentPercent: number;
}

export interface FinTableState {
  tickerList: Array<Ticker>;
  columnsNames: Array<string>;
  columns: Array<string>;
}