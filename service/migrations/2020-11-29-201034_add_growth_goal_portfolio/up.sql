-- Your SQL goes here

----------- tickers
-- Growth ETF            VUG     Stock - Large-Cap Growth      0.04%
INSERT INTO tickers
  (id, symbol, fk_exchange, fee, kind)
  VALUES (DEFAULT, 'VUG', 1, .04, 'STOCK');

-- Mid-Cap Growth ETF    VOT     Stock - Mid-Cap Growth        0.07%
INSERT INTO tickers
  (id, symbol, fk_exchange, fee, kind)
  VALUES (DEFAULT, 'VOT', 1, .07, 'STOCK');

-- Small-Cap Growth ETF  VBK     Stock - Small-Cap Growth      0.07%
INSERT INTO tickers
  (id, symbol, fk_exchange, fee, kind)
  VALUES (DEFAULT, 'VBK', 1, .07, 'STOCK');

-------- goal
INSERT INTO goal_port
  (id, name, description)
  VALUES (DEFAULT, 'Growth Portfolio', 'A collection of Vanguard index funds that represent a growth portfolio. The funds are chosen for their low fees and a diversification between small, medium, large cap.');

INSERT INTO goal_tic
  (id, fk_port_g_id, fk_tic_id, tic_goal_per, ord)
  VALUES (DEFAULT, 2, 12 , 35.0 , 1);
INSERT INTO goal_tic
  (id, fk_port_g_id, fk_tic_id, tic_goal_per, ord)
  VALUES (DEFAULT, 2, 13 , 35.0  , 2);
INSERT INTO goal_tic
  (id, fk_port_g_id, fk_tic_id, tic_goal_per, ord)
  VALUES (DEFAULT, 2, 14 , 30.0  , 3);

