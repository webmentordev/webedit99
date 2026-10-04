-- Add migration script here
CREATE TABLE stores (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    custom_domain VARCHAR(255) UNIQUE,
    currency_id UUID NOT NULL REFERENCES currencies (id),
    timezone VARCHAR(100) NOT NULL,
    default_locale VARCHAR(20) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    domain_verification_token VARCHAR(255) NOT NULL,
    domain_verified_at TIMESTAMPTZ,
    cloudflare_turnstile_site_key VARCHAR(255),
    cloudflare_turnstile_secret_key TEXT,
    contact_email VARCHAR(255),
    contact_phone_number VARCHAR(50),
    whatsapp VARCHAR(50),
    store_address TEXT,
    seo_title VARCHAR(255),
    seo_description TEXT,
    og_title VARCHAR(255),
    og_description TEXT,
    og_image_id UUID,
    jsonld_title VARCHAR(255),
    jsonld_description TEXT,
    jsonld_image_id UUID,
    keywords TEXT,
    twitter VARCHAR(255),
    facebook VARCHAR(255),
    instagram VARCHAR(255),
    linkedin VARCHAR(255),
    pinterest VARCHAR(255),
    favicon_id UUID,
    white_logo_id UUID,
    dark_logo_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_stores_currency_id ON stores (currency_id);

CREATE INDEX idx_stores_og_image_id ON stores (og_image_id);

CREATE INDEX idx_stores_jsonld_image_id ON stores (jsonld_image_id);

CREATE INDEX idx_stores_favicon_id ON stores (favicon_id);

CREATE INDEX idx_stores_white_logo_id ON stores (white_logo_id);

CREATE INDEX idx_stores_dark_logo_id ON stores (dark_logo_id);