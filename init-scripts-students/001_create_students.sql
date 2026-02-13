CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS students (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    telegram_id BIGINT UNIQUE NOT NULL,
    faculty VARCHAR NOT NULL,
    group_name VARCHAR NOT NULL,
    subgroup_name VARCHAR,
    study_form VARCHAR NOT NULL,
    course VARCHAR,
    username VARCHAR,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
