-- -- This file should undo anything in `up.sql`

-----------
DELETE from old_actual_port;
DELETE from actual_tic;
DELETE from actual_port;

-----------
DELETE from goal_tic;
DELETE from goal_port;

-----------
DELETE from tickers;
DELETE from exchanges;

-----------
DELETE from users;
