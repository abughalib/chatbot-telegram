use teloxide::prelude::*;

mod models;
mod answers;

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Starting UP!");

    let bot = Bot::from_env();

    models::Commands::repl(bot, answers::answers).await;
}