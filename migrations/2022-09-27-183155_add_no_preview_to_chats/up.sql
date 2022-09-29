ALTER TABLE telegram_chats ADD COLUMN no_preview BOOLEAN NOT NULL DEFAULT false;
CREATE INDEX telegram_chats_no_preview_index ON telegram_chats(no_preview);