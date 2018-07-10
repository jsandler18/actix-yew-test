-- Your SQL goes here
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    created_on TIMESTAMP NOT NULL,
    item TEXT NOT NULL,
    due TIMESTAMP,
    completed_on TIMESTAMP,
    owner INTEGER REFERENCES users (id) NOT NULL
);