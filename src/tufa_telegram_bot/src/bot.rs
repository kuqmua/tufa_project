use crate::lazy_static::git_info::GIT_INFO;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
pub async fn start_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "show bot source code info ")]
    GitInfo,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    log::info!("answer");
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
        Command::GitInfo => {
            bot.send_message(msg.chat.id, GIT_INFO.get_commit_link())
                .await?
        }
    };

    Ok(())
}
