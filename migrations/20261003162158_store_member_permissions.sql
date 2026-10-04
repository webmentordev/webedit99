-- Add migration script here
CREATE TABLE store_member_permissions (
    id UUID PRIMARY KEY,
    store_member_id UUID NOT NULL REFERENCES store_members (id) ON DELETE CASCADE,
    resource VARCHAR(30) NOT NULL CHECK (
        resource IN (
            'product',
            'brand',
            'collection',
            'order',
            'customer',
            'coupon',
            'blog',
            'page',
            'contact',
            'theme',
            'media',
            'settings'
        )
    ),
    can_view BOOLEAN NOT NULL DEFAULT FALSE,
    can_create BOOLEAN NOT NULL DEFAULT FALSE,
    can_update BOOLEAN NOT NULL DEFAULT FALSE,
    can_delete BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (store_member_id, resource)
);