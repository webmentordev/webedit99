-- Add migration script here
CREATE TABLE store_members (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    is_owner BOOLEAN NOT NULL DEFAULT FALSE,
    is_worker BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (store_id, user_id)
);

CREATE INDEX idx_store_members_user_id ON store_members (user_id);