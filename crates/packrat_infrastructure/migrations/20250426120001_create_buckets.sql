CREATE TABLE buckets (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    location_id BIGINT NOT NULL REFERENCES locations (id) ON DELETE CASCADE
);

CREATE INDEX buckets_location_id_idx ON buckets (location_id);
