#[derive(teloxide::utils::command::BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: std::string::String, age: u8 },
    #[command(description = "show bot source code info ")]
    GitInfo,
}

#[tokio::main]
pub async fn start_bot() {
    let bot = teloxide::Bot::from_env();
    Box::pin(teloxide::repl(bot, |bot: teloxide::Bot, msg: teloxide::types::Message, cmd: Command| async move {
        log::info!("answer");
        let _unused = teloxide::prelude::Requester::send_message(
            &bot,
            msg.chat.id,
            match cmd {
                Command::Help => <Command as teloxide::utils::command::BotCommands>::descriptions().to_string(),
                Command::Username(username) => format!("Your username is @{username}."),
                Command::UsernameAndAge { username, age } => format!("Your username is @{username} and age is {age}."),
                Command::GitInfo => "123message".to_string()
            }
        ).await?;
        Ok(())
    }))
    .await;
}
