ALTER TABLE telegram_subscriptions ADD COLUMN no_preview BOOLEAN NOT NULL DEFAULT false;
CREATE INDEX telegram_subscriptions_no_preview_index ON telegram_subscriptions(no_preview);
