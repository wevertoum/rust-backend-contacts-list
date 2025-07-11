-- to run this script, you can use the following command:
-- psql postgres://{usr}:{pwd}@localhost:5432/contacts_db -f schema.sql

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    genre CHAR(1) NOT NULL CHECK (genre IN ('M', 'F'))
);

-- Create contacts table related to users
CREATE TABLE IF NOT EXISTS contacts (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    user_id UUID NOT NULL UNIQUE,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);
