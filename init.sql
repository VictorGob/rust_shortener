CREATE TABLE IF NOT EXISTS urls_data (
    id VARCHAR(6) PRIMARY KEY UNIQUE,
    url VARCHAR NOT NULL UNIQUE
);