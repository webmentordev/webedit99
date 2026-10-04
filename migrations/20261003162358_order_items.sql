-- Add migration script here
CREATE TABLE order_items (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    product_id UUID REFERENCES products (id) ON DELETE SET NULL,
    product_sub_variant_id UUID REFERENCES product_sub_variants (id) ON DELETE SET NULL,
    sku VARCHAR(100) NOT NULL,
    product_title VARCHAR(255) NOT NULL,
    variant_title VARCHAR(255) NOT NULL,
    sub_variant_title VARCHAR(255),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    unit_price BIGINT NOT NULL,
    line_total BIGINT NOT NULL,
    status VARCHAR(30) NOT NULL DEFAULT 'pending' CHECK (
        status IN (
            'pending',
            'processed',
            'delivered',
            'refunded',
            'faulty_refunded'
        )
    ),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_order_items_order_id ON order_items (order_id);

CREATE INDEX idx_order_items_product_id ON order_items (product_id);

CREATE INDEX idx_order_items_product_sub_variant_id ON order_items (product_sub_variant_id);