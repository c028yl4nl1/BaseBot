use teloxide::{types::CallbackQuery, Bot};

pub async fn handle_callback(bot: Bot, query: CallbackQuery) -> Result<() , Box<dyn std::error::Error + Send + Sync>> {

    Ok(())
}
