#[tokio::main]
pub async fn start_bot() {
    let bot = teloxide::Bot::from_env();
    teloxide::commands_repl(bot, answer, {
        use teloxide::utils::command::BotCommands;
        Command::ty()
    })
    .await;
}

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

async fn answer(bot: teloxide::Bot, msg: teloxide::types::Message, cmd: Command) -> teloxide::requests::ResponseResult<()> {
    log::info!("answer");
    let _unused = match cmd {
        Command::Help => {
            use teloxide::prelude::Requester;
            bot.send_message(
                msg.chat.id,
                {
                    use teloxide::utils::command::BotCommands;
                    Command::descriptions()
                }
                .to_string(),
            )
            .await?
        }
        Command::Username(username) => {
            use teloxide::prelude::Requester;
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            use teloxide::prelude::Requester;
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}.")).await?
        }
        Command::GitInfo => {
            use teloxide::prelude::Requester;
            bot.send_message(msg.chat.id, { "123message" }).await?
        }
    };

    Ok(())
}
