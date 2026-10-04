-- Add migration script here
CREATE TABLE themes (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    default_js TEXT NOT NULL,
    default_css TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_themes_store_id ON themes (store_id);

CREATE UNIQUE INDEX uq_themes_one_active_per_store ON themes (store_id)
WHERE
    is_active = TRUE;