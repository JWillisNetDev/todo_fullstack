ALTER TABLE todo
    DROP COLUMN created_at,
    DROP COLUMN updated_at;

DROP TRIGGER IF EXISTS updated_at ON todo;
DROP FUNCTION IF EXISTS updated_at;