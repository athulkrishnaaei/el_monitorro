use super::Command;
use super::Message;
use crate::bot::telegram_client::Api;
use crate::db::telegram;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

static COMMAND: &str = "/set_preview";
static CALLBACK: &str = "set_preview";
pub struct SetPreview {}

impl SetPreview {
    pub fn execute(db_pool: Pool<ConnectionManager<PgConnection>>, api: Api, message: Message) {
        Self {}.execute(db_pool, api, message);
    }

    pub fn set_no_preview(
        &self,
        db_connection: &mut PgConnection,
        message: &Message,
        params: String,
    ) -> String {
        let vec: Vec<&str> = params.splitn(2, ' ').collect();

        if vec.len() != 2 {
            return "Wrong number of parameters".to_string();
        }

        if vec[1].is_empty() {
            return "Filter can not be empty".to_string();
        }

        if vec[1] != "enable" && vec[1] != "disable" {
            return "Enter enable or disable as parameter".to_string();
        }

        let argument = match vec[1] {
            "enable" => false,
            "disable" => true,
            _ => false,
        };

        let subscription =
            match self.find_subscription(db_connection, message.chat.id, vec[0].to_string()) {
                Err(message) => return message,
                Ok(subscription) => subscription,
            };

        match telegram::set_subscriptions_has_no_preview(db_connection, &subscription, argument) {
            Ok(_) => {
                if vec[1] == "disable" {
                    "Preview Disabled for your messages:".to_string()
                } else {
                    "Preview Enabled for your messages".to_string()
                }
            }
            Err(_) => "Failed to update the filter".to_string(),
        }
    }

    pub fn command() -> &'static str {
        COMMAND
    }

    pub fn callback() -> &'static str {
        CALLBACK
    }
}

impl Command for SetPreview {
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
                self.set_no_preview(&mut connection, message, argument)
            }
            Err(error_message) => error_message,
        }
    }

    fn command(&self) -> &str {
        Self::command()
    }
}
