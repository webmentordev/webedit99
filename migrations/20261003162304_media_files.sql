-- Add migration script here
CREATE TABLE media_files (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    brand_id UUID REFERENCES brands (id) ON DELETE SET NULL,
    collection_id UUID REFERENCES collections (id) ON DELETE SET NULL,
    sub_collection_id UUID REFERENCES sub_collections (id) ON DELETE SET NULL,
    product_id UUID REFERENCES products (id) ON DELETE SET NULL,
    product_sub_variant_id UUID REFERENCES product_sub_variants (id) ON DELETE SET NULL,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    sort_order INTEGER NOT NULL DEFAULT 1,
    file_path VARCHAR(500) NOT NULL,
    disk VARCHAR(50) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    size_bytes BIGINT NOT NULL,
    width INTEGER,
    height INTEGER,
    alt_text VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_media_files_store_id ON media_files (store_id);

CREATE INDEX idx_media_files_brand_id ON media_files (brand_id);

CREATE INDEX idx_media_files_collection_id ON media_files (collection_id);

CREATE INDEX idx_media_files_sub_collection_id ON media_files (sub_collection_id);

CREATE INDEX idx_media_files_product_id ON media_files (product_id);

CREATE INDEX idx_media_files_product_sub_variant_id ON media_files (product_sub_variant_id);