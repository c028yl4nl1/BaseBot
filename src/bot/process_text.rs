use teloxide::prelude::*;
use teloxide::types::{Message , ReactionType};

pub async fn process_text_message(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Verifica se o usuário existe
    if let Some(user) = msg.from {
        let chat_id = msg.chat.id;

        // Adiciona uma reação à mensagem recebida
        bot.set_message_reaction(chat_id, msg.id).reaction(vec![ReactionType::Emoji { emoji: "👌".to_string() }] ).await;
            
     

        log::info!("Reação adicionada à mensagem de {}: {}", user.username.unwrap_or_else(|| user.first_name.clone()), msg.id);
    }

    Ok(())
}
