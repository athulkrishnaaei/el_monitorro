use super::list_subscriptions::Modify;
use super::Close;
use super::Command;
use super::Help;
use super::Response;
use frankenstein::InlineKeyboardButton;
use frankenstein::InlineKeyboardMarkup;
use frankenstein::Message;
use frankenstein::ReplyMarkup;
use frankenstein::SendMessageParams;
use std::str::FromStr;
use typed_builder::TypedBuilder;

static UNKNOWN_COMMAND: &str = "unknown command";
static COMMAND: &str = "/modify_feed";

#[derive(TypedBuilder)]
pub struct ModifyFeed {
    args: String,
    message: Message,
}

impl ModifyFeed {
    pub fn run(&self) {
        self.execute(&self.message);
    }

    fn command_info(&self) -> SendMessageParams {
        let command = Modify::from_str(&self.args).unwrap();

        let send_message_params = self.help_for_command(command);

        let mut buttons: Vec<Vec<InlineKeyboardButton>> = Vec::new();
        let mut row: Vec<InlineKeyboardButton> = Vec::new();

        let button = InlineKeyboardButton::builder()
            .text("Back")
            .callback_data(Help::command().to_string())
            .build();

        row.push(button);
        buttons.push(row);
        buttons.push(Close::button_row());

        send_message_params
    }

    pub fn command() -> &'static str {
        COMMAND
    }

    fn help_for_command(&self, command: Modify) -> SendMessageParams {
        match command {
            Modify::ModifyME => {
                set_list_subcriptions_menu_keyboard(self.message.clone(), self.args.clone())
            }
            Modify::UnknownCommand => unknown_keyboard(self.message.clone()),
        }
    }
}

impl Command for ModifyFeed {
    fn response(&self) -> Response {
        let params = self.command_info();

        Response::Params(params)
    }

    fn send_message(&self, send_message_params: SendMessageParams) {
        if let Err(error) = self.api().send_message_with_params(&send_message_params) {
            error!(
                "Failed to send a message {:?} {:?}",
                error, send_message_params
            );
        }

        self.remove_message(&self.message);
    }
}

pub fn set_list_subcriptions_menu_keyboard(message: Message, feed_id: String) -> SendMessageParams {
    let feed_id = feed_id.replace("/modify_feed", "");
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    let mut row1: Vec<InlineKeyboardButton> = Vec::new();
    let mut row2: Vec<InlineKeyboardButton> = Vec::new();
    let mut row3: Vec<InlineKeyboardButton> = Vec::new();
    let mut row4: Vec<InlineKeyboardButton> = Vec::new();
    let mut row5: Vec<InlineKeyboardButton> = Vec::new();
    let mut row6: Vec<InlineKeyboardButton> = Vec::new();

    let unsubscribe = InlineKeyboardButton::builder()
        .text("Unsubscribe")
        .callback_data(format!("/unsubscribe {}", feed_id))
        .build();

    let remove_filter = InlineKeyboardButton::builder()
        .text("Remove filter ")
        .callback_data(format!("/remove_filter {}", feed_id))
        .build();

    let get_filter = InlineKeyboardButton::builder()
        .text("Get filter ")
        .callback_data(format!("/get_filter {}", feed_id))
        .build();

    let set_template = InlineKeyboardButton::builder()
        .text("Set template")
        .callback_data(format!("set_template {}", feed_id))
        .build();

    let remove_template = InlineKeyboardButton::builder()
        .text("Remove template")
        .callback_data(format!("/remove_template {}", feed_id))
        .build();

    let get_template = InlineKeyboardButton::builder()
        .text("Get template")
        .callback_data(format!("/get_template {}", feed_id))
        .build();

    let set_default_template = InlineKeyboardButton::builder()
        .text("Set default template")
        .callback_data(format!("set_default_template {}", feed_id))
        .build();

    let back_to_menu = InlineKeyboardButton::builder()
        .text("back")
        .callback_data("/list_subscriptions")
        .build();

    row1.push(set_template);
    row2.push(set_default_template);
    row3.push(get_filter);
    row3.push(remove_filter);
    row4.push(get_template);
    row4.push(remove_template);
    row5.push(unsubscribe);
    row6.push(back_to_menu);

    keyboard.push(row1);
    keyboard.push(row2);
    keyboard.push(row3);
    keyboard.push(row4);
    keyboard.push(row5);
    keyboard.push(row6);
    keyboard.push(Close::button_row());

    let inline_keyboard = InlineKeyboardMarkup::builder()
        .inline_keyboard(keyboard)
        .build();
    SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text("Select your option")
        .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
        .build()
}
pub fn unknown_keyboard(message: Message) -> SendMessageParams {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    let mut row1: Vec<InlineKeyboardButton> = Vec::new();

    let unknown = InlineKeyboardButton::builder()
        .text("Unsubscribe")
        .callback_data("unknown".to_string())
        .build();

    row1.push(unknown);

    keyboard.push(row1);

    keyboard.push(Close::button_row());

    let inline_keyboard = InlineKeyboardMarkup::builder()
        .inline_keyboard(keyboard)
        .build();
    SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(UNKNOWN_COMMAND.to_string())
        .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
        .build()
}
