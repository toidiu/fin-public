-- Your SQL goes here

----------- users
INSERT INTO users VALUES
  (DEFAULT, 'toidiu', '123456');

----------- tickers
INSERT INTO exchanges VALUES
  ('NYSE');

INSERT INTO tickers VALUES
  (DEFAULT, 'VTI', 'NYSE', .04, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VTV', 'NYSE', .05, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VOE', 'NYSE', .07, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VBR', 'NYSE', .07, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VEA', 'NYSE', .07, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VWO', 'NYSE', .14, 'STOCK');
INSERT INTO tickers VALUES
  (DEFAULT, 'VTIP', 'NYSE', .06, 'BOND');
INSERT INTO tickers VALUES
  (DEFAULT, 'AGG', 'NYSE', .05, 'BOND');
INSERT INTO tickers VALUES
  (DEFAULT, 'MUB', 'NYSE', .07, 'BOND');
INSERT INTO tickers VALUES
  (DEFAULT, 'BNDX', 'NYSE', .11, 'BOND');
INSERT INTO tickers VALUES
  (DEFAULT, 'VWOB', 'NYSE', .32, 'BOND');

----------- goal
INSERT INTO port_goal VALUES
  (DEFAULT, 58.0, 1.5, 'Value Portfolio');

INSERT INTO tic_goal VALUES
  (1, 1, 20.0, 1);
INSERT INTO tic_goal VALUES
  (1, 2, 6.0, 2);
INSERT INTO tic_goal VALUES
  (1, 3, 4.0, 3);
INSERT INTO tic_goal VALUES
  (1, 4, 3.0, 4);
INSERT INTO tic_goal VALUES
  (1, 5, 15.0, 5);
INSERT INTO tic_goal VALUES
  (1, 6, 10.0, 6);
INSERT INTO tic_goal VALUES
  (1, 7, 3.0, 7);
INSERT INTO tic_goal VALUES
  (1, 8, 4.0, 8);
INSERT INTO tic_goal VALUES
  (1, 9, 14.0, 9);
INSERT INTO tic_goal VALUES
  (1, 10, 12.0, 10);
INSERT INTO tic_goal VALUES
  (1, 11, 9.0, 11);

----------- actual
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 1, 2.0);
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 2, 1.0);
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 3, 1.0);
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 4, 1.0);
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 5, 3.0);
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 6, 2.0);
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 7, 1.0);
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 8, 1.0);
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 9, 1.0);
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 10, 2.0);
INSERT INTO tic_actual VALUES
  (DEFAULT, 1, 1, 11, 1.0);

