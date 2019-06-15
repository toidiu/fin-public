-- -- Your SQL goes here

----------- users
INSERT INTO users VALUES
  (DEFAULT, 'apoorv@toidiu.com', '$$scrypt$ln=14,r=8,p=1$Xvtx1zK+bcot3cjKfZR9+A$fAM+i/wFUfQaq+HQe2RgsyjgrT93cz/jUkPV+zRNF4I');

INSERT INTO users VALUES
  (DEFAULT, 'test@toidiu.com', '$$scrypt$ln=14,r=8,p=1$Xvtx1zK+bcot3cjKfZR9+A$fAM+i/wFUfQaq+HQe2RgsyjgrT93cz/jUkPV+zRNF4I');

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

INSERT INTO actual_port
  (id, fk_user_id, fk_port_g_id, stock_percent, deviation, version, last_updated)
  VALUES (DEFAULT, 1, 1, 58.0, 1.5, 1, now());
INSERT INTO actual_port VALUES
  (DEFAULT, 1, 1, 90.0, 1.5, 1, now());
INSERT INTO actual_port VALUES
  (DEFAULT, 2, 1, 50.0, 1.5, 1, now());

INSERT INTO actual_tic
  (id, fk_port_g_id, fk_port_a_id, fk_tic_id, actual_shares)
  VALUES (DEFAULT, 1, 1, 1, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 1, 2, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 1, 3, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 1, 4, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 1, 5, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 1, 6, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 1, 7, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 1, 8, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 1, 9, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 1, 10, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 1, 11, 0.0);

INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 1, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 2, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 3, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 4, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 5, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 6, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 7, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 8, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 9, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 10, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 2, 11, 0.0);

INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 1, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 2, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 3, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 4, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 5, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 6, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 7, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 8, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 9, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 10, 0.0);
INSERT INTO actual_tic VALUES (DEFAULT, 1, 3, 11, 0.0);
