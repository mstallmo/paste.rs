-- This file should undo anything in `up.sql`
ALTER TABLE 'pastes' ALTER COLUMN 'hash' set not null;