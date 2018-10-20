-- This file should undo anything in `up.sql`

-----------
DROP TABLE tic_actual;
-- DROP TABLE port_actual;

-----------
DROP TABLE tic_goal;
DROP TABLE port_goal;

-----------
DROP TABLE tickers;
DROP DOMAIN dom_tic_kind;
DROP TABLE exchanges;

-----------
DROP TABLE users;
