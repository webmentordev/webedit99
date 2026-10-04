-- Add migration script here
CREATE TABLE blog_posts (
    id UUID PRIMARY KEY,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    blog_category_id UUID REFERENCES blog_categories (id) ON DELETE SET NULL,
    author_user_id UUID NOT NULL REFERENCES users (id),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    excerpt TEXT,
    content TEXT NOT NULL,
    featured_image_id UUID REFERENCES media_files (id) ON DELETE SET NULL,
    seo_title VARCHAR(255),
    seo_description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (store_id, slug)
);

CREATE INDEX idx_blog_posts_blog_category_id ON blog_posts (blog_category_id);

CREATE INDEX idx_blog_posts_author_user_id ON blog_posts (author_user_id);

CREATE INDEX idx_blog_posts_featured_image_id ON blog_posts (featured_image_id);