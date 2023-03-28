use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Commands {
    #[command(description = "get help")]
    Help,
    #[command(description = "Generate image from text")]
    Imagine(String),
    #[command(description = "Chat with bot")]
    Chat(String),
}