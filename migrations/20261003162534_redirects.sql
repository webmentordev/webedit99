-- Add migration script here
CREATE TABLE redirects (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    from_path VARCHAR(500) NOT NULL,
    to_path VARCHAR(500) NOT NULL,
    status_code SMALLINT NOT NULL DEFAULT 301,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (store_id, from_path)
);