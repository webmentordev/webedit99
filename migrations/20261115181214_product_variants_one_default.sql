-- Add migration script here
CREATE UNIQUE INDEX idx_product_variants_one_default ON product_variants (product_id)
WHERE
    is_default;