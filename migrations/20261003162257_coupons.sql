-- Add migration script here
CREATE TABLE coupons (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    code VARCHAR(100) NOT NULL,
    off_percent NUMERIC(5, 2) NOT NULL CHECK (
        off_percent > 0
        AND off_percent <= 100
    ),
    customer_id UUID REFERENCES customers (id) ON DELETE CASCADE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,
    total_uses INTEGER NOT NULL DEFAULT 0,
    max_total_uses INTEGER,
    max_uses_per_customer INTEGER NOT NULL DEFAULT 1,
    min_order_amount BIGINT,
    max_discount_amount BIGINT,
    starts_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    UNIQUE (store_id, code),
    CHECK (
        max_total_uses IS NULL
        OR total_uses <= max_total_uses
    )
);

CREATE INDEX idx_coupons_customer_id ON coupons (customer_id);