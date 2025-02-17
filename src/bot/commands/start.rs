use super::Command;
use super::Message;
use super::Response;
use typed_builder::TypedBuilder;

static START: &str =
        "El Monitorro is feed reader as a Telegram bot.\n\
         It supports RSS, Atom and JSON feeds.\n\n\
         Use /help to see available commands.\n\n\
         Synchronization information.\n\
         When you subscribe to a new feed, you'll receive 10 last messages from it. After that, you'll start receiving only new feed items.\n\
         Feed updates check interval is 1 minute. Unread items delivery interval is also 1 minute.\n\
         Currently, the number of subscriptions is limited to 20.\n\n\
         Join https://t.me/el_monitorro or contact the author (@Ayrat555) with your feedback, suggestions, found bugs, etc. The bot is open source. You can find it at https://github.com/ayrat555/el_monitorro\n\n\
         Unlike other similar projects, El Monitorro is completely open and it's free of charge. I develop it in my free time and pay for hosting myself. Please support the bot by donating - https://paypal.me/AyratBadykov, BTC - bc1q94ru65c8pg87ghhjlc7fteuxncpyj8e28cxf42";

static COMMAND: &str = "/start";

#[derive(TypedBuilder)]
pub struct Start {
    message: Message,
}

impl Start {
    pub fn run(&self) {
        self.execute(&self.message);
    }

    pub fn command() -> &'static str {
        COMMAND
    }
}

impl Command for Start {
    fn response(&self) -> Response {
        Response::Simple(START.to_string())
    }
}
