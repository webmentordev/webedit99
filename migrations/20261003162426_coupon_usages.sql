-- Add migration script here
CREATE TABLE coupon_usages (
    id UUID PRIMARY KEY,
    coupon_id UUID NOT NULL REFERENCES coupons (id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customers (id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_coupon_usages_coupon_id ON coupon_usages (coupon_id);

CREATE INDEX idx_coupon_usages_customer_id ON coupon_usages (customer_id);

CREATE INDEX idx_coupon_usages_order_id ON coupon_usages (order_id);