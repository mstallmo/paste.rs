-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS create_pastes_hash ON pastes;
DROP FUNCTION IF EXISTS pastes_create_hash;