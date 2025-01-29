use teloxide::{types::{ChatMemberUpdated, Message}, Bot};

pub async fn add_gruop(bot: Bot, msg: ChatMemberUpdated) -> Result<() , Box<dyn std::error::Error + Send + Sync>>{

    Ok(())
}
