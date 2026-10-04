-- Add migration script here
CREATE TABLE order_notes (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users (id),
    note TEXT NOT NULL,
    is_internal BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_order_notes_store_id ON order_notes (store_id);

CREATE INDEX idx_order_notes_order_id ON order_notes (order_id);

CREATE INDEX idx_order_notes_user_id ON order_notes (user_id);