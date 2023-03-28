use crate::models::Commands;
use std::str::FromStr;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
use url::Url;

pub async fn answers(bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
    match cmd {
        Commands::Help => {
            bot.send_message(msg.chat.id, Commands::descriptions().to_string())
                .await?
        }
        Commands::Imagine(prompt) => {
            let some_image: InputFile =
                InputFile::url(Url::from_str("https://i.imgur.com/1Z5QY9A.jpg").unwrap());
            bot.send_photo(msg.chat.id, some_image)
                .caption(prompt)
                .await?
        }
        Commands::Chat(prompt) => {
            bot.send_message(msg.chat.id, "Your Entered: ".to_string() + &prompt)
                .await?
        }
    };

    Ok(())
}
