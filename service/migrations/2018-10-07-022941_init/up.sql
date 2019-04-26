-- Your SQL goes here

CREATE TABLE users (
  id bigserial PRIMARY KEY,
  email text UNIQUE NOT NULL,
  password text NOT NULL,
  CHECK(length(password) >= 6)
);

-----------
CREATE TABLE exchanges (
  id serial  PRIMARY KEY,
  mic text
);

CREATE DOMAIN dom_tic_kind
  AS text NOT NULL
  CONSTRAINT tk_check CHECK (VALUE IN('STOCK', 'BOND'));

CREATE TABLE tickers (
  id bigserial PRIMARY KEY,
  symbol text NOT NULL,
  fk_exchange int4 NOT NULL REFERENCES exchanges(id),
  fee float4 NOT NULL,
  kind dom_tic_kind NOT NULL,
  UNIQUE (symbol, fk_exchange)
);

-----------
CREATE TABLE goal_port (
  id bigserial PRIMARY KEY,
  name text NOT NULL,
  description text DEFAULT ''
);

CREATE TABLE goal_tic (
  id bigserial PRIMARY KEY,
  fk_port_g_id int8 NOT NULL REFERENCES goal_port(id),
  fk_tic_id int8 NOT NULL REFERENCES tickers(id),
  tic_goal_per float4 NOT NULL,
  ord int NOT NULL,
  UNIQUE(fk_port_g_id, ord),
  UNIQUE(fk_port_g_id, fk_tic_id)
);

-----------
CREATE TABLE actual_port (
  id bigserial PRIMARY KEY,
  fk_user_id int8 NOT NULL REFERENCES users(id),
  fk_port_g_id int8 NOT NULL REFERENCES goal_port(id),
  stock_percent float4 NOT NULL,
  deviation float4 NOT NULL,
  version int4 NOT NULL,
  last_updated timestamptz NOT NULL,
  UNIQUE(id, fk_port_g_id) -- needed for FK in actual_tic
);

CREATE TABLE actual_tic (
  id bigserial PRIMARY KEY,
  fk_port_g_id int8 NOT NULL,
  fk_port_a_id int8 NOT NULL,
  fk_tic_id int8 NOT NULL,
  actual_shares float8 NOT NULL,
  -- both actual_port and goal_tic should have the same fk_port_g_id
  FOREIGN KEY(fk_port_g_id, fk_tic_id) REFERENCES goal_tic(fk_port_g_id, fk_tic_id),
  FOREIGN KEY(fk_port_g_id, fk_port_a_id) REFERENCES actual_port(fk_port_g_id, id),
  UNIQUE(fk_port_a_id, fk_tic_id, fk_port_g_id)
);

-----------
CREATE DOMAIN dom_port_action
  AS text NOT NULL
  CONSTRAINT tk_check CHECK (VALUE IN('TICKER', 'PERCENT'));

CREATE TABLE old_actual_port (
  id bigserial PRIMARY KEY,
  fk_port_a_id int8 NOT NULL REFERENCES actual_port(id),
  version int4 NOT NULL,
  init_port_a_data jsonb,
  new_port_a_data jsonb,
  actions_data jsonb,
  port_action dom_port_action NOT NULL,
  UNIQUE(fk_port_a_id, version)
);

