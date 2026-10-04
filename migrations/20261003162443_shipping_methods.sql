-- Add migration script here
CREATE TABLE shipping_methods (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    price BIGINT NOT NULL,
    estimated_days INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    sort_order INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_shipping_methods_store_id ON shipping_methods (store_id);