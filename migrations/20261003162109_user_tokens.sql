-- Add migration script here
CREATE TABLE user_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
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

CREATE INDEX idx_user_tokens_user_id ON user_tokens (user_id);

CREATE INDEX idx_user_tokens_token_hash ON user_tokens (token_hash);