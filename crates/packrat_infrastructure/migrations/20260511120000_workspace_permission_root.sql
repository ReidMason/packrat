INSERT INTO permissions (parent_id, name, slug, description)
VALUES (
    NULL,
    'Workspace full access',
    'workspace.all',
    'Full access to all workspace capabilities; underlying tenant and assets trees are children.'
);

UPDATE permissions
SET
    parent_id = (
        SELECT id
        FROM permissions
        WHERE
            slug = 'workspace.all'
    )
WHERE
    slug IN ('tenant.admin', 'assets.admin');

DELETE FROM role_permissions
WHERE
    role_id = (
        SELECT id
        FROM roles
        WHERE
            name = 'Owner'
            AND tenant_id IS NULL
        LIMIT
            1
    );

INSERT INTO role_permissions (role_id, permission_id)
SELECT
    r.id,
    p.id
FROM roles r
    INNER JOIN permissions p ON p.slug = 'workspace.all'
WHERE
    r.name = 'Owner'
    AND r.tenant_id IS NULL;
