-- Add migration script here
ALTER TABLE stores
ADD CONSTRAINT fk_stores_og_image_id FOREIGN KEY (og_image_id) REFERENCES media_files (id) ON DELETE SET NULL,
ADD CONSTRAINT fk_stores_jsonld_image_id FOREIGN KEY (jsonld_image_id) REFERENCES media_files (id) ON DELETE SET NULL,
ADD CONSTRAINT fk_stores_favicon_id FOREIGN KEY (favicon_id) REFERENCES media_files (id) ON DELETE SET NULL,
ADD CONSTRAINT fk_stores_white_logo_id FOREIGN KEY (white_logo_id) REFERENCES media_files (id) ON DELETE SET NULL,
ADD CONSTRAINT fk_stores_dark_logo_id FOREIGN KEY (dark_logo_id) REFERENCES media_files (id) ON DELETE SET NULL;