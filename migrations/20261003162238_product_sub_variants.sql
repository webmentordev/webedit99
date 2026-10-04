-- Add migration script here
CREATE TABLE product_sub_variants (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    product_variant_id UUID NOT NULL REFERENCES product_variants (id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    sku VARCHAR(100) NOT NULL,
    price BIGINT NOT NULL,
    compare_at_price BIGINT,
    cost_price BIGINT,
    quantity INTEGER,
    low_stock_threshold INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    sort_order INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (store_id, sku)
);

CREATE INDEX idx_product_sub_variants_product_variant_id ON product_sub_variants (product_variant_id);