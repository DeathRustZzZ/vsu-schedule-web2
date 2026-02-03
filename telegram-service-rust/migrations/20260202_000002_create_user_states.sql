CREATE TABLE IF NOT EXISTS user_states (
    id SERIAL PRIMARY KEY,
    telegram_id BIGINT UNIQUE NOT NULL,
    state VARCHAR DEFAULT 'idle',
    faculty VARCHAR,
    study_form VARCHAR,
    course VARCHAR,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
