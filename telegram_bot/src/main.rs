use git_info::{GetGitCommitLink as _, PROJECT_GIT_INFO};
use optml::Optml;
use std::borrow::Cow;
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
#[allow(clippy::single_call_fn)] // command text is centralized for deterministic unit tests
fn cmd_rsp_text(cmd: &Cmd) -> Cow<'static, str> {
    match cmd {
        Cmd::Help => Cow::Owned(<Cmd as BotCommands>::descriptions().to_string()),
        Cmd::Username(username) => Cow::Owned(format!("Your username is @{username}.")),
        Cmd::UsernameAndAge { username, age } => {
            Cow::Owned(format!("Your username is @{username} and age is {age}."))
        }
        Cmd::GitInfo => Cow::Owned(PROJECT_GIT_INFO.get_git_commit_link()),
    }
}
#[tokio::main]
async fn main() {
    Box::pin(repl(
        Bot::from_env(),
        async |bot: Bot, msg: Message, cmd: Cmd| {
            log::info!("answer");
            let sent_msg_42913f2d =
                Requester::send_message(&bot, msg.chat.id, cmd_rsp_text(&cmd)).await?;
            drop(sent_msg_42913f2d);
            Ok(())
        },
    ))
    .await;
}
#[cfg(test)]
mod tests {
    use super::{Cmd, cmd_rsp_text};
    #[test]
    fn cmd_rsp_text_for_username_is_expected() {
        assert_eq!(
            cmd_rsp_text(&Cmd::Username("alice".to_owned())).as_ref(),
            "Your username is @alice."
        );
    }
    #[test]
    fn cmd_rsp_text_for_username_and_age_is_expected() {
        assert_eq!(
            cmd_rsp_text(&Cmd::UsernameAndAge {
                username: "alice".to_owned(),
                age: 27,
            })
            .as_ref(),
            "Your username is @alice and age is 27."
        );
    }
    #[test]
    fn cmd_rsp_text_for_git_info_contains_commit_link() {
        let rsp = cmd_rsp_text(&Cmd::GitInfo);
        assert!(!rsp.as_ref().is_empty());
    }
    #[test]
    fn cmd_rsp_text_for_help_contains_description_header() {
        assert!(
            cmd_rsp_text(&Cmd::Help)
                .as_ref()
                .contains("These cmds are supported:")
        );
    }
}
