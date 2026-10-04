-- Add migration script here
CREATE TABLE carts (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers (id) ON DELETE CASCADE,
    session_token VARCHAR(255),
    coupon_id UUID REFERENCES coupons (id) ON DELETE SET NULL,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_carts_store_id ON carts (store_id);

CREATE INDEX idx_carts_customer_id ON carts (customer_id);

CREATE INDEX idx_carts_coupon_id ON carts (coupon_id);

CREATE INDEX idx_carts_session_token ON carts (session_token);