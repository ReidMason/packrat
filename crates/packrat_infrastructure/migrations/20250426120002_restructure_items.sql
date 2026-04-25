ALTER TABLE items
    ADD COLUMN location_id BIGINT REFERENCES locations (id) ON DELETE CASCADE,
    ADD COLUMN bucket_id BIGINT REFERENCES buckets (id) ON DELETE CASCADE;

UPDATE items
SET
    location_id = (SELECT id FROM locations WHERE name = 'Default' LIMIT 1),
    bucket_id = NULL
WHERE
    location_id IS NULL;

ALTER TABLE items
    ADD CONSTRAINT item_exactly_one_parent CHECK (
        (location_id IS NOT NULL AND bucket_id IS NULL)
        OR (location_id IS NULL AND bucket_id IS NOT NULL)
    );

CREATE INDEX items_by_location_idx ON items (location_id)
WHERE
    bucket_id IS NULL;

CREATE INDEX items_by_bucket_idx ON items (bucket_id)
WHERE
    bucket_id IS NOT NULL;
