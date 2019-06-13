-- This file should undo anything in `up.sql`

-----------
DROP TABLE old_actual_port;
DROP DOMAIN dom_port_action;
DROP TABLE actual_tic;
DROP TABLE actual_port;

-----------
DROP TABLE goal_tic;
DROP TABLE goal_port;

-----------
DROP TABLE tickers;
DROP DOMAIN dom_tic_kind;
DROP TABLE exchanges;

-----------
DROP TABLE users;
