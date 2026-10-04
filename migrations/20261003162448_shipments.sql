-- Add migration script here
CREATE TABLE shipments (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    shipping_method_id UUID REFERENCES shipping_methods (id) ON DELETE SET NULL,
    carrier VARCHAR(100),
    tracking_number VARCHAR(255),
    tracking_url VARCHAR(500),
    shipped_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_shipments_order_id ON shipments (order_id);

CREATE INDEX idx_shipments_shipping_method_id ON shipments (shipping_method_id);