use std::str::FromStr;

use teloxide::{macros::BotCommands, utils::command::ParseError};

#[derive(Debug, Clone)]
pub struct StartArgsInner {
    pub code: String,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct StartArgs(pub Option<StartArgsInner>);

impl FromStr for StartArgs {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(StartArgs(None));
        }

        let Some((code, username)) = s.split_once('_') else {
            return Err(ParseError::IncorrectFormat(
                "Invalid command format.".into(),
            ));
        };

        Ok(StartArgs(Some(StartArgsInner {
            code: code.to_string(),
            username: username.to_string(),
        })))
    }
}

#[derive(BotCommands, Debug, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "display the start message.")]
    Start(StartArgs),
    #[command(
        description = "add an extra forwarder code to your account. You can get this code by examining the URL shown by the app after pressing Save."
    )]
    Add { code: String },
    #[command(description = "display the linked codes.")]
    Linked,
    #[command(description = "display this help message.")]
    Help,
}
