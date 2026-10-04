-- Add migration script here
CREATE TABLE currencies (
    id UUID PRIMARY KEY,
    currency_code VARCHAR(10) NOT NULL UNIQUE,
    symbol VARCHAR(10) NOT NULL,
    decimal_places SMALLINT NOT NULL DEFAULT 2,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);