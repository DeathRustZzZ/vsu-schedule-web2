ALTER TABLE user_states
    ADD COLUMN IF NOT EXISTS ui_chat_id BIGINT,
    ADD COLUMN IF NOT EXISTS ui_message_id INT,
    ADD COLUMN IF NOT EXISTS reply_message_id INT,
    ADD COLUMN IF NOT EXISTS reply_state VARCHAR;
