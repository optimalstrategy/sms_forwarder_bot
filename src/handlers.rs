use teloxide::{
    prelude::*,
    types::{Message, ParseMode},
    utils::command::BotCommands,
};

use crate::{
    commands::{Command, StartArgs, StartArgsInner},
    queries::BotUser,
    setup::AppContext,
};

pub async fn handle(context: AppContext, msg: Message, cmd: Command) -> anyhow::Result<()> {
    let from_username = msg.from.as_ref().and_then(|u| u.username.as_ref()).cloned();
    if msg.from.is_none() || from_username.is_none() {
        return Ok(());
    }

    match cmd {
        Command::Help => {
            context
                .bot
                .send_message(msg.chat.id, Command::descriptions().to_string())
                .send()
                .await?;
        }
        Command::Start(StartArgs(None)) => {
            context.bot.send_message(msg.chat.id, r#"
                Install the <a href="https://github.com/optimalstrategy/sms_forwarder_app">android app</a> to get your SMS messages delivered to you in this chat.
            "#.trim()
        )
                .parse_mode(ParseMode::Html)
                .send()
                .await?;
        }
        Command::Start(StartArgs(Some(args))) => {
            handle_add_code(context, msg.chat.id, from_username.unwrap(), args, true).await?;
        }
        Command::Add { code } => {
            handle_add_code(
                context,
                msg.chat.id,
                from_username.clone().unwrap(),
                StartArgsInner {
                    code,
                    username: from_username.unwrap(),
                },
                false,
            )
            .await?
        }
        Command::Linked => {
            display_linked_codes(context, msg.chat.id).await?;
        }
    };

    Ok(())
}

async fn handle_add_code(
    context: AppContext,
    chat_id: ChatId,
    from_username: String,
    args: StartArgsInner,
    clear_codes: bool,
) -> anyhow::Result<()> {
    let mut tx = context.db.begin().await?;
    let mut user = crate::queries::find_user_by_chat_id(&mut *tx, chat_id)
        .await?
        .unwrap_or_else(|| BotUser {
            chat_id,
            username: from_username.clone().to_lowercase(),
            codes: Default::default(),
        });

    if args.username != from_username.to_lowercase() {
        context
            .bot
            .send_message(chat_id, "You cannot set up the bot for others.")
            .send()
            .await?;
        return Ok(());
    }

    user.username = args.username;
    if clear_codes {
        user.codes.clear();
    }
    user.codes.insert(args.code);
    crate::queries::create_user(&mut *tx, user).await?;
    tx.commit().await?;

    context
        .bot
        .send_message(chat_id, "Done! You are ready to receive notifications.")
        .await?;

    Ok(())
}

async fn display_linked_codes(context: AppContext, chat_id: ChatId) -> anyhow::Result<()> {
    let user =
        crate::queries::find_user_by_chat_id(&mut *context.db.acquire().await?, chat_id).await?;

    let message = if let Some(user) = user {
        let mut codes = user.codes.into_iter().collect::<Vec<_>>();
        codes.sort();
        if codes.is_empty() {
            "You haven't set up any devices yet.".to_string()
        } else {
            format!(
                "Your linked devices:\n{}",
                codes
                    .into_iter()
                    .enumerate()
                    .map(|(i, code)| format!("{}. {}", i + 1, code))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    } else {
        "You haven't set up any devices yet.".to_string()
    };

    context.bot.send_message(chat_id, message).send().await?;

    Ok(())
}
