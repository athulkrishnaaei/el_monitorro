ALTER TABLE telegram_chats ADD COLUMN disable_notification BOOLEAN NOT NULL DEFAULT false;
CREATE INDEX telegram_chats_notification_index ON telegram_chats(disable_notification);