CREATE TABLE sessions (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token_hash BYTEA NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    CONSTRAINT sessions_token_hash_len_chk CHECK (octet_length(token_hash) = 32),
    CONSTRAINT sessions_expires_after_created_chk CHECK (expires_at > created)
);

CREATE UNIQUE INDEX sessions_token_hash_key ON sessions (token_hash);

CREATE INDEX sessions_user_id_idx ON sessions (user_id);
