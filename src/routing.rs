use crate::{commands::Commands, help};
use teloxide::prelude::*;

pub async fn routing(bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
    println!("{:?}", "Received message");
    match cmd {
        Commands::HelpCommand(_) => help::help(bot, msg).await,
    }

    Ok(())
}
