-- Add migration script here
CREATE TABLE menu_items (
    id UUID PRIMARY KEY,
    navigation_menu_id UUID NOT NULL REFERENCES navigation_menus (id) ON DELETE CASCADE,
    parent_id UUID REFERENCES menu_items (id) ON DELETE CASCADE,
    label VARCHAR(255) NOT NULL,
    url VARCHAR(500),
    linkable_type VARCHAR(20) CHECK (
        linkable_type IN (
            'product',
            'collection',
            'page',
            'blog_post'
        )
    ),
    linkable_id UUID,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    sort_order INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_menu_items_navigation_menu_id ON menu_items (navigation_menu_id);

CREATE INDEX idx_menu_items_parent_id ON menu_items (parent_id);

CREATE INDEX idx_menu_items_linkable ON menu_items (linkable_type, linkable_id);