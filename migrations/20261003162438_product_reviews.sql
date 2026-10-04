-- Add migration script here
CREATE TABLE product_reviews (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customers (id) ON DELETE CASCADE,
    order_id UUID REFERENCES orders (id) ON DELETE SET NULL,
    stars SMALLINT NOT NULL CHECK (stars BETWEEN 1 AND 5),
    subject VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    is_approved BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (customer_id, product_id)
);

CREATE INDEX idx_product_reviews_store_id ON product_reviews (store_id);

CREATE INDEX idx_product_reviews_product_id ON product_reviews (product_id);

CREATE INDEX idx_product_reviews_order_id ON product_reviews (order_id);