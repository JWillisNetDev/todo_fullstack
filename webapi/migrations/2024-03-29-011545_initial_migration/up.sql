CREATE TABLE todo (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    is_completed BOOLEAN NOT NULL DEFAULT FALSE
);