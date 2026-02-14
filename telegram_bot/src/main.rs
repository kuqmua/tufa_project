use teloxide::{Bot, prelude::Requester, repl, types::Message, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "show bot source code info ")]
    GitInfo,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

#[tokio::main]
async fn main() {
    Box::pin(repl(
        Bot::from_env(),
        async |bot: Bot, msg: Message, cmd: Command| {
            log::info!("answer");
            let _unused = Requester::send_message(
                &bot,
                msg.chat.id,
                match cmd {
                    Command::Help => <Command as BotCommands>::descriptions().to_string(),
                    Command::Username(username) => format!("Your username is @{username}."),
                    Command::UsernameAndAge { username, age } => {
                        format!("Your username is @{username} and age is {age}.")
                    }
                    Command::GitInfo => "123message".to_owned(),
                },
            )
            .await?;
            Ok(())
        },
    ))
    .await;
}
