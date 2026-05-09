CREATE TABLE tags (
    id BIGSERIAL PRIMARY KEY,
    tenant_id BIGINT NOT NULL REFERENCES tenants (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    normalized TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT tags_tenant_normalized_unique UNIQUE (tenant_id, normalized)
);

CREATE INDEX tags_tenant_id_idx ON tags (tenant_id);

CREATE TABLE asset_tags (
    asset_id BIGINT NOT NULL REFERENCES assets (id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    PRIMARY KEY (asset_id, tag_id)
);

CREATE INDEX asset_tags_tag_id_idx ON asset_tags (tag_id);
