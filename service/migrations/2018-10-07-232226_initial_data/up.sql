-- -- Your SQL goes here

----------- users

----------- tickers
INSERT INTO exchanges VALUES
  (DEFAULT, 'NYSE Arca');
INSERT INTO exchanges VALUES
  (DEFAULT, 'NASDAQ Global Market');

INSERT INTO tickers VALUES
  (DEFAULT, 'VTI', '1', .04, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VTV', '1', .05, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VOE', '1', .07, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VBR', '1', .07, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VEA', '1', .07, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VWO', '1', .14, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VTIP', '2', .06, 'BOND');
INSERT INTO tickers VALUES
  (DEFAULT, 'AGG', '1', .05, 'BOND');
INSERT INTO tickers VALUES
  (DEFAULT, 'MUB', '1', .07, 'BOND');
INSERT INTO tickers VALUES
  (DEFAULT, 'BNDX', '2', .11, 'BOND');
INSERT INTO tickers VALUES
  (DEFAULT, 'VWOB', '2', .32, 'BOND');

----------- goal
INSERT INTO goal_port VALUES
  (DEFAULT, 'Value Portfolio', 'A collection of funds that represent a value portfolio. The funds are chosen for their low fees.');

INSERT INTO goal_tic VALUES (DEFAULT, 1, 1  , 35.0 , 1);
INSERT INTO goal_tic VALUES (DEFAULT, 1, 2  , 9.4  , 2);
INSERT INTO goal_tic VALUES (DEFAULT, 1, 3  , 7.7  , 3);
INSERT INTO goal_tic VALUES (DEFAULT, 1, 4  , 6.5  , 4);
INSERT INTO goal_tic VALUES (DEFAULT, 1, 5  , 25.9 , 5);
INSERT INTO goal_tic VALUES (DEFAULT, 1, 6  , 15.2 , 6);
INSERT INTO goal_tic VALUES (DEFAULT, 1, 7  , 6.8  , 7);
INSERT INTO goal_tic VALUES (DEFAULT, 1, 8  , 8.4  , 8);
INSERT INTO goal_tic VALUES (DEFAULT, 1, 9  , 36   , 9);
INSERT INTO goal_tic VALUES (DEFAULT, 1, 10 , 30.4 , 10);
INSERT INTO goal_tic VALUES (DEFAULT, 1, 11 , 18.6 , 11);

----------- actual

