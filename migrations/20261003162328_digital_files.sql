-- Add migration script here
CREATE TABLE digital_files (
    id UUID PRIMARY KEY,
    product_id UUID NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    product_sub_variant_id UUID REFERENCES product_sub_variants (id) ON DELETE CASCADE,
    media_file_id UUID NOT NULL REFERENCES media_files (id) ON DELETE CASCADE,
    download_limit INTEGER,
    expires_after_days INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_digital_files_product_id ON digital_files (product_id);

CREATE INDEX idx_digital_files_product_sub_variant_id ON digital_files (product_sub_variant_id);

CREATE INDEX idx_digital_files_media_file_id ON digital_files (media_file_id);