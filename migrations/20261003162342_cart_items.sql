-- Add migration script here
CREATE TABLE cart_items (
    id UUID PRIMARY KEY,
    cart_id UUID NOT NULL REFERENCES carts (id) ON DELETE CASCADE,
    product_sub_variant_id UUID NOT NULL REFERENCES product_sub_variants (id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_cart_items_cart_id ON cart_items (cart_id);

CREATE INDEX idx_cart_items_product_sub_variant_id ON cart_items (product_sub_variant_id);