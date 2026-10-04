-- Add migration script here
CREATE TABLE inventory_movements (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    product_sub_variant_id UUID NOT NULL REFERENCES product_sub_variants (id) ON DELETE CASCADE,
    change INTEGER NOT NULL,
    reason VARCHAR(20) NOT NULL CHECK (
        reason IN (
            'restock',
            'sale',
            'refund',
            'adjustment',
            'damaged'
        )
    ),
    order_id UUID REFERENCES orders (id) ON DELETE SET NULL,
    user_id UUID REFERENCES users (id) ON DELETE SET NULL,
    note TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_inventory_movements_store_id ON inventory_movements (store_id);

CREATE INDEX idx_inventory_movements_product_sub_variant_id ON inventory_movements (product_sub_variant_id);

CREATE INDEX idx_inventory_movements_order_id ON inventory_movements (order_id);

CREATE INDEX idx_inventory_movements_user_id ON inventory_movements (user_id);