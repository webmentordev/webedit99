-- Add migration script here
CREATE TABLE products (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    brand_id UUID REFERENCES brands (id) ON DELETE SET NULL,
    description TEXT NOT NULL,
    content TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 1,
    is_digital BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,
    min_order_qty INTEGER,
    max_order_qty INTEGER,
    has_warranty BOOLEAN NOT NULL DEFAULT FALSE,
    warranty_type VARCHAR(10) CHECK (
        warranty_type IN (
            'day',
            'hour',
            'month',
            'year'
        )
    ),
    warranty_value INTEGER,
    has_inventory BOOLEAN NOT NULL DEFAULT FALSE,
    seo_title VARCHAR(255),
    seo_description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    UNIQUE (store_id, slug)
);

CREATE INDEX idx_products_brand_id ON products (brand_id);