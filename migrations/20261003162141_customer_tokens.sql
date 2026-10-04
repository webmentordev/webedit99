-- Add migration script here
CREATE TABLE customer_tokens (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL REFERENCES customers (id) ON DELETE CASCADE,
    type VARCHAR(30) NOT NULL CHECK (
        type IN (
            'email_verification',
            'password_reset'
        )
    ),
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_customer_tokens_customer_id ON customer_tokens (customer_id);

CREATE INDEX idx_customer_tokens_token_hash ON customer_tokens (token_hash);