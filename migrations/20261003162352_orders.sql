-- Add migration script here
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    order_number VARCHAR(50) NOT NULL,
    customer_id UUID NOT NULL REFERENCES customers (id),
    status VARCHAR(30) NOT NULL DEFAULT 'payment_pending' CHECK (
        status IN (
            'payment_pending',
            'processing',
            'processed',
            'in_transit',
            'delivered',
            'cancelled',
            'failed',
            'refunding',
            'refunded'
        )
    ),
    payment_status VARCHAR(30) NOT NULL DEFAULT 'unpaid' CHECK (
        payment_status IN (
            'unpaid',
            'paid',
            'partially_refunded',
            'refunded',
            'failed'
        )
    ),
    payment_method VARCHAR(100) NOT NULL,
    currency_id UUID NOT NULL REFERENCES currencies (id),
    subtotal BIGINT NOT NULL DEFAULT 0,
    discount_total BIGINT NOT NULL DEFAULT 0,
    shipping_total BIGINT NOT NULL DEFAULT 0,
    tax_total BIGINT NOT NULL DEFAULT 0,
    total_price_paid BIGINT NOT NULL DEFAULT 0,
    coupon_id UUID REFERENCES coupons (id) ON DELETE SET NULL,
    coupon_note TEXT,
    coupon_off_percent NUMERIC(5, 2),
    customer_note TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (store_id, order_number)
);

CREATE INDEX idx_orders_customer_id ON orders (customer_id);

CREATE INDEX idx_orders_currency_id ON orders (currency_id);

CREATE INDEX idx_orders_coupon_id ON orders (coupon_id);

CREATE INDEX idx_orders_status ON orders (status);

CREATE INDEX idx_orders_payment_status ON orders (payment_status);