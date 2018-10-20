-- Your SQL goes here

CREATE TABLE users (
  id bigserial PRIMARY KEY,
  username text UNIQUE NOT NULL,
  password text NOT NULL,
  CHECK(length(password) >= 6)
);

-----------
CREATE TABLE exchanges (
  name text PRIMARY KEY -- NYSE, NASDAQ
);

CREATE DOMAIN dom_tic_kind
  AS text NOT NULL
  CONSTRAINT tk_check CHECK (VALUE IN('STOCK', 'BOND'));

CREATE TABLE tickers (
  id bigserial PRIMARY KEY,
  symbol text NOT NULL,
  fk_exchange text REFERENCES exchanges(name) NOT NULL,
  fee float4 NOT NULL,
  kind dom_tic_kind NOT NULL,
  UNIQUE (symbol, fk_exchange)
);

-----------
CREATE TABLE port_goal (
  id bigserial PRIMARY KEY,
  stock_per float4 NOT NULL,
  deviation float4 NOT NULL,
  name text NOT NULL,
  description text DEFAULT ''
);

CREATE TABLE tic_goal (
  fk_port_g_id int8 REFERENCES port_goal(id) NOT NULL,
  fk_tic_id int8 REFERENCES tickers(id) NOT NULL,
  goal_per float4 NOT NULL,
  ord int NOT NULL,
  UNIQUE(fk_port_g_id, ord),
  PRIMARY KEY (fk_port_g_id, fk_tic_id)
);

-----------
CREATE TABLE tic_actual (
  id bigserial PRIMARY KEY,
  fk_user_id int8 NOT NULL REFERENCES users(id),
  fk_port_g_id int8 NOT NULL REFERENCES port_goal(id),
  fk_tic_id int8 NOT NULL REFERENCES tickers(id),
  actual_shares float4 NOT NULL DEFAULT 0.0,
  UNIQUE(fk_user_id, fk_port_g_id, fk_tic_id)
);



