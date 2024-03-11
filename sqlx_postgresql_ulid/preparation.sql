BEGIN;

CREATE DOMAIN user_id AS VARCHAR(26);

CREATE TABLE users (
    id         user_id PRIMARY KEY DEFAULT generate_ulid(),
    name       TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (name) VALUES ('alpha'), ('beta');

SELECT * FROM users;

COMMIT;
