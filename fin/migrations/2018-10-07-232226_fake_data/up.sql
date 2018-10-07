-- Your SQL goes here

----------- users
INSERT INTO users VALUES
  (DEFAULT, 'toidiu', '123456');

----------- tickers
INSERT INTO exchanges VALUES
  ('NYSE');

INSERT INTO tickers VALUES
  (DEFAULT, 'vti', 'NYSE', .04, 'stock');
INSERT INTO tickers VALUES
  (DEFAULT, 'vtv', 'NYSE', .05, 'stock');
INSERT INTO tickers VALUES
  (DEFAULT, 'voe', 'NYSE', .07, 'stock');
INSERT INTO tickers VALUES
  (DEFAULT, 'vbr', 'NYSE', .07, 'stock');
INSERT INTO tickers VALUES
  (DEFAULT, 'vea', 'NYSE', .07, 'stock');
INSERT INTO tickers VALUES
  (DEFAULT, 'vwo', 'NYSE', .14, 'stock');
INSERT INTO tickers VALUES
  (DEFAULT, 'vtip', 'NYSE', .06, 'stock');
INSERT INTO tickers VALUES
  (DEFAULT, 'agg', 'NYSE', .05, 'stock');
INSERT INTO tickers VALUES
  (DEFAULT, 'mub', 'NYSE', .07, 'stock');
INSERT INTO tickers VALUES
  (DEFAULT, 'bndx', 'NYSE', .11, 'stock');
INSERT INTO tickers VALUES
  (DEFAULT, 'vwob', 'NYSE', .32, 'stock');

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

