-- -- Your SQL goes here

----------- users
INSERT INTO users VALUES
  (DEFAULT, 'apoorv@toidiu.com', '$$scrypt$ln=14,r=8,p=1$Xvtx1zK+bcot3cjKfZR9+A$fAM+i/wFUfQaq+HQe2RgsyjgrT93cz/jUkPV+zRNF4I');

INSERT INTO users VALUES
  (DEFAULT, 'test@toidiu.com', '$$scrypt$ln=14,r=8,p=1$Xvtx1zK+bcot3cjKfZR9+A$fAM+i/wFUfQaq+HQe2RgsyjgrT93cz/jUkPV+zRNF4I');


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
