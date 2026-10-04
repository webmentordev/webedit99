-- Add migration script here
CREATE TABLE store_logs (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    user_id UUID REFERENCES users (id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (
        status IN (
            'success',
            'failed',
            'warning',
            'info'
        )
    ),
    ip_address VARCHAR(45),
    context JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_store_logs_store_id ON store_logs (store_id);

CREATE INDEX idx_store_logs_user_id ON store_logs (user_id);