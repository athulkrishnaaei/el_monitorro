use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use frankenstein::{
    InlineKeyboardButton, InlineKeyboardMarkup, Message, ReplyMarkup, SendMessageParams,
};

use crate::bot::handler::Handler;

static NOTIFICATION: &str = "/set_notification";
static PREVIEW: &str = "/set_preview";

pub struct SetNotificationPreviewKeyboard {}

impl SetNotificationPreviewKeyboard {
    pub fn set_notification_preview_menu_keyboard(
        message: Message,
        feed_id: String,
    ) -> SendMessageParams {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

        let mut row1: Vec<InlineKeyboardButton> = Vec::new();
        let mut row2: Vec<InlineKeyboardButton> = Vec::new();
        let mut row3: Vec<InlineKeyboardButton> = Vec::new();

        let substring = InlineKeyboardButton::builder()
            .text("Enable/Disable Message Notification")
            .callback_data(format!("set_notification {}", feed_id))
            .build();

        let bold = InlineKeyboardButton::builder()
            .text("Enable/Disable Message Preview")
            .callback_data(format!("set_preview {}", feed_id))
            .build();

        let back_to_subscription_list_keyboard = InlineKeyboardButton::builder()
            .text("Back to menu 🔙 ")
            .callback_data(format!("list_subscriptions {}", feed_id)) //used letter s to identify the callback ,callback data support no of characters
            .build();

        row1.push(substring);
        row2.push(bold);
        row3.push(back_to_subscription_list_keyboard);

        keyboard.push(row1);
        keyboard.push(row2);
        keyboard.push(row3);

        let inline_keyboard = InlineKeyboardMarkup::builder()
            .inline_keyboard(keyboard)
            .build();
        SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text("Select your option")
            .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
            .build()
    }
    pub fn select_feed_url_keyboard_notification_preview(
        message: Message,
        feed_ids: std::str::Split<'_, char>,
        db_pool: Pool<ConnectionManager<PgConnection>>,
    ) -> SendMessageParams {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

        for feed in feed_ids.clone() {
            let mut row: Vec<InlineKeyboardButton> = Vec::new();
            let name = format!(
                "{} ",
                Handler::get_feed_url_by_id(db_pool.clone(), feed.to_string())
            );
            let unsubscribe_inlinekeyboard = InlineKeyboardButton::builder()
                .text(name.clone())
                .callback_data(format!("set_template {}", feed)) //used letter s to identify the callback ,callback data support no of characters
                .build();

            row.push(unsubscribe_inlinekeyboard);
            keyboard.push(row);
        }

        let inline_keyboard = InlineKeyboardMarkup::builder()
            .inline_keyboard(keyboard)
            .build();

        SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text("Select feed url to modify")
            .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
            .build()
    }
    pub fn set_notification_keyboard(message: Message, feed_id: String) -> SendMessageParams {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

        let mut row1: Vec<InlineKeyboardButton> = Vec::new();
        let mut row2: Vec<InlineKeyboardButton> = Vec::new();
        let mut row3: Vec<InlineKeyboardButton> = Vec::new();

        let enable_message_notification = InlineKeyboardButton::builder()
            .text("Enable Message Notification")
            .callback_data(format!("/set_notification {} enable", feed_id))
            .build();
        let disable_message_notification = InlineKeyboardButton::builder()
            .text("Disable Message Notification")
            .callback_data(format!("/set_notification {} disable", feed_id))
            .build();
        let back_to_menu = InlineKeyboardButton::builder()
            .text("Back to menus 🔙 ")
            .callback_data(format!("list_subscriptions {}", feed_id))
            .build();

        row1.push(enable_message_notification);
        row2.push(disable_message_notification);
        row3.push(back_to_menu);

        keyboard.push(row1);
        keyboard.push(row2);
        keyboard.push(row3);
        let inline_keyboard = InlineKeyboardMarkup::builder()
            .inline_keyboard(keyboard)
            .build();
        SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text("Select your option")
            .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
            .build()
    }
    pub fn set_preview_keyboard(message: Message, feed_id: String) -> SendMessageParams {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

        let mut row1: Vec<InlineKeyboardButton> = Vec::new();
        let mut row2: Vec<InlineKeyboardButton> = Vec::new();
        let mut row3: Vec<InlineKeyboardButton> = Vec::new();

        let enable_message_preview = InlineKeyboardButton::builder()
            .text("Enable Message Preview")
            .callback_data(format!("/set_preview {} enable", feed_id))
            .build();
        let disable_message_preview = InlineKeyboardButton::builder()
            .text("Disable Message Preview")
            .callback_data(format!("/set_preview {} disable", feed_id))
            .build();
        let back_to_menu = InlineKeyboardButton::builder()
            .text("Back to menus 🔙 ")
            .callback_data(format!("list_subscriptions {}", feed_id))
            .build();

        row1.push(enable_message_preview);
        row2.push(disable_message_preview);
        row3.push(back_to_menu);

        keyboard.push(row1);
        keyboard.push(row2);
        keyboard.push(row3);
        let inline_keyboard = InlineKeyboardMarkup::builder()
            .inline_keyboard(keyboard)
            .build();
        SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text("Select your option")
            .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
            .build()
    }

    pub fn notification() -> &'static str {
        NOTIFICATION
    }

    pub fn preview() -> &'static str {
        PREVIEW
    }
}
