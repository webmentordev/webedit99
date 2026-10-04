-- Add migration script here
CREATE TABLE payments (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    provider VARCHAR(100) NOT NULL,
    transaction_id VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (
        status IN (
            'pending',
            'paid',
            'failed',
            'refunded'
        )
    ),
    amount BIGINT NOT NULL,
    currency_id UUID NOT NULL REFERENCES currencies (id),
    raw_response JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_payments_store_id ON payments (store_id);

CREATE INDEX idx_payments_order_id ON payments (order_id);

CREATE INDEX idx_payments_currency_id ON payments (currency_id);

CREATE INDEX idx_payments_transaction_id ON payments (transaction_id);