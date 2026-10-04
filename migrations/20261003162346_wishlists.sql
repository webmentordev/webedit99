-- Add migration script here
CREATE TABLE wishlists (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL REFERENCES customers (id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (customer_id, product_id)
);

CREATE INDEX idx_wishlists_product_id ON wishlists (product_id);