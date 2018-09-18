-- Your SQL goes here
CREATE FUNCTION pastes_create_hash() RETURNS trigger AS $$
    BEGIN
        NEW.hash := id_encode(NEW.id, 'the answer is 42', 10);
        RETURN NEW;
    END;
$$LANGUAGE plpgsql;

CREATE TRIGGER create_pastes_hash BEFORE INSERT ON pastes FOR EACH ROW EXECUTE PROCEDURE pastes_create_hash();
