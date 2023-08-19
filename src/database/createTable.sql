-- users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    balance DECIMAL NOT NULL DEFAULT 0.0
);

-- expenses table
CREATE TABLE expenses (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    amount DECIMAL NOT NULL,
    expense_name VARCHAR NOT NULL,
    expense_category VARCHAR,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);