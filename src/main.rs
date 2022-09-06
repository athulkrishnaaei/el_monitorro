use diesel::PgConnection;
use dotenv::dotenv;
use el_monitorro::bot;
use el_monitorro::config::Config;
use fang::Queue;

// use frankenstein::Api;
use frankenstein::KeyboardButton;
use frankenstein::ReplyKeyboardMarkup;
use frankenstein::ReplyMarkup;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
// use frankenstein::TelegramApi;
// use crate::bot::telegram_client::Api;


// static TOKEN: &str = "https://api.telegram.org/bot5725562379:AAHyPc8l-yyXs99NnYpfXLMw4tzcJwmFr5E";
static CHAT_ID: i64 = 275_808_073;

fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let queue = Queue::builder()
        .connection_pool(el_monitorro::db::pool().clone())
        .build();

    if Config::all_binaries() {
        el_monitorro::start_clean_workers(&queue);
        el_monitorro::start_sync_workers(&queue);
        el_monitorro::start_delivery_workers(&queue);
    }

    el_monitorro::start_scheduler(&queue);

    let api = bot::telegram_client::Api::new();

    let mut keyboard: Vec<Vec<KeyboardButton>> = Vec::new();

    for i in 1..5 {
        let mut row: Vec<KeyboardButton> = Vec::new();

        for j in 1..5 {
            let name = format!("{}{}", i, j);
            let button = KeyboardButton::builder().text(name).build();

            row.push(button);
        }

        keyboard.push(row);
    }

    let keyboard_markup = ReplyKeyboardMarkup::builder().keyboard(keyboard).build();

    let send_message_params = SendMessageParams::builder()
        .chat_id(CHAT_ID)
        .text("/start")
        .reply_markup(ReplyMarkup::ReplyKeyboardMarkup(keyboard_markup))
        .build();

   bot::handler::Handler::start();
    
    api.send_message(&send_message_params).unwrap();

    

    


    
}
