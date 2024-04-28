use std::str::FromStr;
use teloxide::types::BotCommand;
use teloxide::utils::command::{BotCommands, CommandDescription, CommandDescriptions};

#[derive(Clone)]
pub enum Commands {
    HelpCommand(HelpCommandType),
}

#[derive(Clone)]
pub enum HelpCommandType {
    Help,
}

impl BotCommands for Commands {
    fn parse(s: &str, _: &str) -> Result<Self, teloxide::utils::command::ParseError> {
        let a = s.trim().to_lowercase();
        if a == "/help" {
            Ok(Commands::HelpCommand(HelpCommandType::Help))
        } else {
            let error = String::from_str("Hi").unwrap();
            Err(teloxide::utils::command::ParseError::UnknownCommand(error))
        }
    }
    fn descriptions() -> teloxide::utils::command::CommandDescriptions<'static> {
        CommandDescriptions::new(&[
            CommandDescription {
                prefix: "/",
                command: "start",
                description: "start this bot",
            },
            CommandDescription {
                prefix: "/",
                command: "help",
                description: "show this message",
            },
        ])
    }
    fn bot_commands() -> Vec<teloxide::types::BotCommand> {
        let command = String::from_str("Hi").unwrap();
        let description = String::from_str("Hi").unwrap();
        let other: teloxide::types::BotCommand = BotCommand::new(command, description);
        let a: Vec<teloxide::types::BotCommand> = vec![other];
        a
    }
}
