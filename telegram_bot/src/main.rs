use optml::Optml;
use teloxide::{Bot, prelude::Requester, repl, types::Message, utils::command::BotCommands};
#[derive(BotCommands, Clone, Optml)]
#[command(rename_rule = "lowercase", description = "These cmds are supported:")]
enum Cmd {
    #[command(description = "show bot src code info ")]
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
        async |bot: Bot, msg: Message, cmd: Cmd| {
            log::info!("answer");
            let _unused = Requester::send_message(
                &bot,
                msg.chat.id,
                match cmd {
                    Cmd::Help => <Cmd as BotCommands>::descriptions().to_string(),
                    Cmd::Username(username) => format!("Your username is @{username}."),
                    Cmd::UsernameAndAge { username, age } => {
                        format!("Your username is @{username} and age is {age}.")
                    }
                    Cmd::GitInfo => "123message".to_owned(),
                },
            )
            .await?;
            Ok(())
        },
    ))
    .await;
}
