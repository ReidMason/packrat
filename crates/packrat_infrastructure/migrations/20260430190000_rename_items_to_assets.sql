-- Rename legacy `items` table to `assets` (must run after `20260430180551_delete_entity.sql`).
-- Idempotent: no-op if `items` no longer exists (e.g. already renamed).
DO $$
BEGIN
    IF to_regclass('public.items') IS NOT NULL THEN
        ALTER TABLE items RENAME TO assets;
    END IF;
END
$$;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
          AND c.relkind = 'i'
          AND c.relname = 'items_parent_id_idx'
    ) THEN
        ALTER INDEX items_parent_id_idx RENAME TO assets_parent_id_idx;
    END IF;
END
$$;
