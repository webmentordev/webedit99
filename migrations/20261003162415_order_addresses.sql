-- Add migration script here
CREATE TABLE order_addresses (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    type VARCHAR(20) NOT NULL CHECK (
        type IN ('shipping', 'billing')
    ),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(50),
    address_1 VARCHAR(255) NOT NULL,
    address_2 VARCHAR(255),
    postal_code VARCHAR(20) NOT NULL,
    city VARCHAR(100) NOT NULL,
    country VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_order_addresses_order_id ON order_addresses (order_id);

CREATE INDEX idx_order_addresses_store_id ON order_addresses (store_id);