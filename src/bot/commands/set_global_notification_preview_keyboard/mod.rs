use frankenstein::{
    InlineKeyboardButton, InlineKeyboardMarkup, Message, ReplyMarkup, SendMessageParams,
};

static NOTIFICATION: &str = "/set_global_notification";
static PREVIEW: &str = "/set_global_preview";

pub struct SetGlobalNotificationPreviewKeyboard {}

impl SetGlobalNotificationPreviewKeyboard {
    // pub fn set_global_notification_preview_menu_keyboard(message: Message, feed_id: String) -> SendMessageParams {
    //     let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    //     let mut row1: Vec<InlineKeyboardButton> = Vec::new();
    //     let mut row2: Vec<InlineKeyboardButton> = Vec::new();
    //     let mut row3: Vec<InlineKeyboardButton> = Vec::new();

    //     let global_notification = InlineKeyboardButton::builder()
    //         .text("Enable/Disable Message Notification")
    //         .callback_data("set_global_notification")
    //         .build();

    //     let global_preview = InlineKeyboardButton::builder()
    //         .text("Enable/Disable Message Preview")
    //         .callback_data("set_global_preview {}")
    //         .build();

    //     let back_to_subscription_list_keyboard = InlineKeyboardButton::builder()
    //         .text("Back to menu 🔙 ")
    //         .callback_data(format!("list_subscriptions {}", feed_id)) //used letter s to identify the callback ,callback data support no of characters
    //         .build();

    //     row1.push(global_notification);
    //     row2.push(global_preview);
    //     row3.push(back_to_subscription_list_keyboard);

    //     keyboard.push(row1);
    //     keyboard.push(row2);
    //     keyboard.push(row3);

    //     let inline_keyboard = InlineKeyboardMarkup::builder()
    //         .inline_keyboard(keyboard)
    //         .build();
    //     SendMessageParams::builder()
    //         .chat_id(message.chat.id)
    //         .text("Select your option")
    //         .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
    //         .build()
    // }

    // pub fn set_global_notification_preview_menu_keyboard(message: Message, feed_id: String) -> SendMessageParams {
    //     let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    //     let mut row1: Vec<InlineKeyboardButton> = Vec::new();
    //     let mut row2: Vec<InlineKeyboardButton> = Vec::new();

    //     let global_preview = InlineKeyboardButton::builder()
    //         .text("Enable/Disable Message Preview")
    //         .callback_data("set_global_preview {}")
    //         .build();

    //     let back_to_subscription_list_keyboard = InlineKeyboardButton::builder()
    //         .text("Back to menu 🔙 ")
    //         .callback_data(format!("list_subscriptions {}", feed_id)) //used letter s to identify the callback ,callback data support no of characters
    //         .build();

    //     row1.push(global_preview);
    //     row2.push(back_to_subscription_list_keyboard);

    //     keyboard.push(row2);
    //     keyboard.push(row3);

    //     let inline_keyboard = InlineKeyboardMarkup::builder()
    //         .inline_keyboard(keyboard)
    //         .build();
    //     SendMessageParams::builder()
    //         .chat_id(message.chat.id)
    //         .text("Select your option")
    //         .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
    //         .build()
    // }

    pub fn set_global_notification_keyboard(message: Message) -> SendMessageParams {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

        let mut row1: Vec<InlineKeyboardButton> = Vec::new();
        let mut row2: Vec<InlineKeyboardButton> = Vec::new();
        let mut row3: Vec<InlineKeyboardButton> = Vec::new();

        let enable_message_notification = InlineKeyboardButton::builder()
            .text("Enable Message Notification")
            .callback_data("/set_global_notification enable")
            .build();
        let disable_message_notification = InlineKeyboardButton::builder()
            .text("Disable Message Notification")
            .callback_data("/set_global_notification disable")
            .build();
        let back_to_menu = InlineKeyboardButton::builder()
            .text("Back to menus 🔙 ")
            .callback_data("/set_global_notification")
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
    pub fn set_global_preview_keyboard(message: Message) -> SendMessageParams {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

        let mut row1: Vec<InlineKeyboardButton> = Vec::new();
        let mut row2: Vec<InlineKeyboardButton> = Vec::new();
        let mut row3: Vec<InlineKeyboardButton> = Vec::new();

        let enable_message_preview = InlineKeyboardButton::builder()
            .text("Enable Message Preview")
            .callback_data("/set_global_preview enable")
            .build();
        let disable_message_preview = InlineKeyboardButton::builder()
            .text("Disable Message Preview")
            .callback_data("/set_global_preview disable")
            .build();
        let back_to_menu = InlineKeyboardButton::builder()
            .text("Back to menus 🔙 ")
            .callback_data("/set_global_notification")
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
