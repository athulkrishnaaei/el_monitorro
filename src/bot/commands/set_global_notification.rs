use super::Command;
use super::Message;
use crate::bot::telegram_client::Api;
use crate::db::telegram;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

static COMMAND: &str = "/set_global_notification";
static CALLBACK: &str = "set_global_notification";
pub struct SetGlobalNotification {}

impl SetGlobalNotification {
    pub fn execute(db_pool: Pool<ConnectionManager<PgConnection>>, api: Api, message: Message) {
        Self {}.execute(db_pool, api, message);
    }

    fn set_global_notification(
        &self,
        db_connection: &mut PgConnection,
        message: &Message,
        params: String,
    ) -> String {
        let chat = match telegram::find_chat(db_connection, message.chat.id) {
            Some(chat) => chat,
            None => return "You don't have any subcriptions".to_string(),
        };

        if params.is_empty() {
            return "Filter can not be empty".to_string();
        }
        if params != "disable" && params != "enable" {
            return "Enter enable or disable as parameter".to_string();
        }

        let argument = match params.as_str() {
            "enable" => false,
            "disable" => true,
            _ => false,
        };
        match telegram::set_global_notification(db_connection, &chat, argument) {
            Ok(_) => {
                if params == "disable" {
                    "The messages will have no notification:".to_string()
                } else {
                    "The messages will have notification".to_string()
                }
            }
            Err(_) => "Failed to update the preview".to_string(),
        }
    }

    pub fn command() -> &'static str {
        COMMAND
    }

    pub fn callback() -> &'static str {
        CALLBACK
    }
}

impl Command for SetGlobalNotification {
    fn response(
        &self,
        db_pool: Pool<ConnectionManager<PgConnection>>,
        message: &Message,
        _api: &Api,
    ) -> String {
        match self.fetch_db_connection(db_pool) {
            Ok(mut connection) => {
                let text = message.text.as_ref().unwrap();
                let argument = self.parse_argument(text);
                self.set_global_notification(&mut connection, message, argument)
            }
            Err(error_message) => error_message,
        }
    }

    fn command(&self) -> &str {
        Self::command()
    }
}
