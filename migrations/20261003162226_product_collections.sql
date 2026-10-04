-- Add migration script here
CREATE TABLE product_collections (
    id UUID PRIMARY KEY,
    product_id UUID NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    collection_id UUID NOT NULL REFERENCES collections (id) ON DELETE CASCADE,
    sub_collection_id UUID REFERENCES sub_collections (id) ON DELETE CASCADE,
    sort_order INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE NULLS NOT DISTINCT (
        product_id,
        collection_id,
        sub_collection_id
    )
);

CREATE INDEX idx_product_collections_collection_id ON product_collections (collection_id);

CREATE INDEX idx_product_collections_sub_collection_id ON product_collections (sub_collection_id);