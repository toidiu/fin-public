-- This file should undo anything in `up.sql`

DELETE FROM exchanges
  where (name = 'fake');

