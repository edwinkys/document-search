CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS version (
    id BOOL PRIMARY KEY DEFAULT true,
    version TEXT NOT NULL,
    CONSTRAINT single_row_table CHECK (id)
);

INSERT INTO version (version)
VALUES ('0.1.0');

CREATE TABLE IF NOT EXISTS namespaces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX IF NOT EXISTS namespaces_name_idx ON namespaces (name);

CREATE TYPE doc_status
AS ENUM ('pending', 'processing', 'completed', 'failed');
