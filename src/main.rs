mod commands;
mod help;
mod routing;

use commands::Commands;
use teloxide::prelude::Bot;
use teloxide::repls::CommandReplExt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");
    println!("{:?}", "Received ");

    let bot = Bot::from_env();

    //teloxide::repl(bot, roll_dice::roll_dice).await;
    Commands::repl(bot, routing::routing).await;
}
