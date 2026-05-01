-- Schema history anchor: version 20260425120003 was briefly present in the tree without a file,
-- which caused `VersionMissing(20260425120003)` for databases that recorded this version.
-- This step is intentionally a no-op. The `items` → `assets` rename runs in
-- `20260430190000_rename_items_to_assets.sql` (after `20260430180551_delete_entity.sql`).
--
-- If you instead see `VersionMismatch(20260425120003)`, your DB has a different checksum stored
-- for this version (e.g. you applied an older rename script here). Fix once with:
--   DELETE FROM _sqlx_migrations WHERE version = 20260425120003;
-- then run migrations again.
SELECT 1;
