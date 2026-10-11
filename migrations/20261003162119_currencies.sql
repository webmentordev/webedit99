-- Add migration script here
CREATE TABLE currencies (
    id UUID PRIMARY KEY,
    currency_code VARCHAR(10) NOT NULL UNIQUE,
    symbol VARCHAR(10) NOT NULL,
    decimal_places SMALLINT NOT NULL DEFAULT 2,
    value NUMERIC(20, 10) NOT NULL CHECK (value > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO
    currencies (
        id,
        currency_code,
        symbol,
        decimal_places,
        value
    )
VALUES (
        gen_random_uuid (),
        'USD',
        '$',
        2,
        1
    );