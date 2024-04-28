use teloxide::prelude::Bot;
use teloxide::prelude::Message;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use crate::commands::Commands;

pub async fn help(bot: Bot, msg: Message) {
    let help_text = Commands::descriptions().to_string();
    let _ = bot.send_message(msg.chat.id, help_text).await;
}
