use std::collections::HashSet;

use sqlx::PgExecutor;
use teloxide::types::ChatId;

#[derive(sqlx::FromRow)]
struct BotUserRow {
    pub telegram_id: i64,
    pub username: String,
    pub codes: String,
}

#[derive(Debug, Clone)]
pub struct BotUser {
    pub chat_id: ChatId,
    pub username: String,
    pub codes: HashSet<String>,
}

impl From<BotUserRow> for BotUser {
    fn from(row: BotUserRow) -> Self {
        Self {
            chat_id: ChatId(row.telegram_id),
            username: row.username,
            codes: row.codes.split(',').map(String::from).collect(),
        }
    }
}

impl From<BotUser> for BotUserRow {
    fn from(user: BotUser) -> Self {
        Self {
            telegram_id: user.chat_id.0,
            username: user.username,
            codes: user.codes.into_iter().collect::<Vec<_>>().join(","),
        }
    }
}

pub async fn find_user_by_chat_id(
    tx: impl PgExecutor<'_>,
    chat_id: ChatId,
) -> Result<Option<BotUser>, sqlx::Error> {
    let row = sqlx::query_as::<_, BotUserRow>("SELECT * FROM bot_tguser WHERE telegram_id = $1")
        .bind(chat_id.0)
        .fetch_optional(tx)
        .await?;

    Ok(row.map(BotUser::from))
}

pub async fn find_user_by_username(
    tx: impl PgExecutor<'_>,
    username: &str,
) -> Result<Option<BotUser>, sqlx::Error> {
    let row = sqlx::query_as::<_, BotUserRow>("SELECT * FROM bot_tguser WHERE username = $1")
        .bind(username)
        .fetch_optional(tx)
        .await?;

    Ok(row.map(BotUser::from))
}

pub async fn create_user(tx: impl PgExecutor<'_>, user: BotUser) -> Result<(), sqlx::Error> {
    let BotUserRow {
        codes,
        telegram_id,
        username,
    } = user.clone().into();

    sqlx::query(
        "
        INSERT INTO bot_tguser (telegram_id, username, codes) VALUES ($1, $2, $3) 
        ON CONFLICT (telegram_id) 
            DO UPDATE SET 
                codes = EXCLUDED.codes, 
                username = EXCLUDED.username
    ",
    )
    .bind(telegram_id)
    .bind(username)
    .bind(codes)
    .execute(tx)
    .await?;

    Ok(())
}
