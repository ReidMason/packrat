ALTER TABLE assets
ADD COLUMN tenant_id BIGINT NOT NULL REFERENCES tenants (id) ON DELETE CASCADE;

CREATE INDEX assets_tenant_id_idx ON assets (tenant_id);
