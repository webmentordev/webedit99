-- Add migration script here
CREATE TABLE navigation_menus (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    location VARCHAR(20) NOT NULL CHECK (
        location IN ('header', 'footer', 'other')
    ),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_navigation_menus_store_id ON navigation_menus (store_id);