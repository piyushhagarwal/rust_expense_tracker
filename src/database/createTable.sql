-- users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    balance FLOAT8 NOT NULL DEFAULT 0.0
);

-- expenses table
CREATE TABLE expenses (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    amount FLOAT8 NOT NULL,
    expense_name VARCHAR NOT NULL,
    expense_category VARCHAR,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);