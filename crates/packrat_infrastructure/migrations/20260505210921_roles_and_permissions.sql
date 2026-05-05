CREATE TABLE tenants (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE permissions (
    id BIGSERIAL PRIMARY KEY,
    parent_id BIGINT REFERENCES permissions (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE roles (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    tenant_id BIGINT REFERENCES tenants (id) ON DELETE CASCADE,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX roles_tenant_id_name_idx ON roles (tenant_id, name) WHERE tenant_id IS NOT NULL;
CREATE UNIQUE INDEX roles_name_idx ON roles (name) WHERE tenant_id IS NULL;

CREATE TABLE user_tenants (
    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    tenant_id BIGINT NOT NULL REFERENCES tenants (id) ON DELETE CASCADE,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, tenant_id)
);

CREATE TABLE user_roles (
    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES roles (id) ON DELETE CASCADE,
    tenant_id BIGINT NOT NULL REFERENCES tenants (id) ON DELETE CASCADE,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id, tenant_id)
);

CREATE TABLE user_permissions (
    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES permissions (id) ON DELETE CASCADE,
    tenant_id BIGINT NOT NULL REFERENCES tenants (id) ON DELETE CASCADE,
    expires TIMESTAMPTZ,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, permission_id, tenant_id)
);

CREATE TABLE role_permissions (
    role_id BIGINT NOT NULL REFERENCES roles (id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES permissions (id) ON DELETE CASCADE,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (role_id, permission_id)
);

-- Tenant permissions
WITH ta AS (
    INSERT INTO permissions (parent_id, name, slug, description)
    VALUES (
        NULL,
        'Tenant administration',
        'tenant.admin',
        'Manage members, settings, and billing for the workspace'
    )
    RETURNING id
)
INSERT INTO permissions (parent_id, name, slug, description)
SELECT
    ta.id,
    v.name,
    v.slug,
    v.descr
FROM ta
    CROSS JOIN LATERAL (
        VALUES
            (
                'Manage members',
                'tenant.members',
                'Invite and remove members, assign roles'
            ),
            (
                'Manage settings',
                'tenant.settings',
                'Change workspace name and configuration'
            ),
            (
                'Manage billing',
                'tenant.billing',
                'Manage subscription and payment method'
            )
    ) AS v(name, slug, descr);

-- Assets permissions
WITH aa AS (
    INSERT INTO permissions (parent_id, name, slug, description)
    VALUES (
        NULL,
        'Assets administration',
        'assets.admin',
        'Create, read, update, and delete assets'
    )
    RETURNING id
)
INSERT INTO permissions (parent_id, name, slug, description)
SELECT
    aa.id,
    v.name,
    v.slug,
    v.descr
FROM aa
    CROSS JOIN LATERAL (
        VALUES
            (
                'View assets',
                'assets.read',
                'List, search, and view assets'
            ),
            (
                'Edit assets',
                'assets.write',
                'Create and update assets'
            ),
            (
                'Delete assets',
                'assets.delete',
                'Remove or soft-delete assets'
            )
    ) AS v(name, slug, descr);

-- Global template roles
INSERT INTO roles (name, tenant_id) VALUES
    ('Owner', NULL),
    ('Admin', NULL),
    ('Member', NULL),
    ('Viewer', NULL);

-- Owner role permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
    INNER JOIN permissions p ON p.slug IN ('tenant.admin', 'assets.admin')
WHERE r.name = 'Owner';

-- Admin role permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
    INNER JOIN permissions p ON p.slug IN (
        'tenant.members',
        'tenant.settings',
        'tenant.billing',
        'assets.admin'
    )
WHERE r.name = 'Admin';

-- Member role permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
    INNER JOIN permissions p ON p.slug IN ('assets.read', 'assets.write')
WHERE r.name = 'Member';

-- Viewer role permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
    INNER JOIN permissions p ON p.slug = 'assets.read'
WHERE r.name = 'Viewer';
