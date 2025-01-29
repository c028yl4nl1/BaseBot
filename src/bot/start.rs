use crate::jsonconfig::json::JsonBot;
use crate::openfilemax;
use teloxide::prelude::*;
use teloxide::types::*;
use teloxide::dptree::entry;
use super::add_bot_in_grupo::add_gruop;
use super::process_callback::handle_callback;
use super::process_text::process_text_message;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
use log::info;
pub async fn start_bot() -> Result<(), Error> {
    if let Err(e) = openfilemax::set_max_open_files(65535) {
        eprintln!("Erro ao ajustar o limite de arquivos abertos: {}", e);
    }
    let bot = teloxide::Bot::new(JsonBot::set_json(JsonBot::TokenBot
    ).as_str().unwrap());


    info!("Token correto , inicializando");
    let schema = Update::filter_message()
        .filter_map(|update: Update| update.from().cloned())
        .branch(
            Message::filter_text().endpoint(|bot, msg: Message| async move {
                // Cria uma nova tarefa para cada mensagem recebida
                tokio::spawn(
                    async move { if let Err(e) = process_text_message(bot, msg).await {} },
                );

                Ok::<(), Error>(()) // Retorna Ok() para o endpoint
            }),
        );

    let schema2 = Update::filter_callback_query().endpoint(|bot, query| async move {
        if let Err(e) = handle_callback(bot, query).await {
            eprintln!("Error ao processar callback: {}", e);
        } else {
            // println!("{}", "Foi clicado");
        }
        Ok::<(), Error>(())
    });

    let schema3 =
        Update::filter_my_chat_member().endpoint(|bot: Bot, msg: ChatMemberUpdated| async move {
            add_gruop(bot, msg).await;
            Ok::<(), Error>(())
        });

    loop {
        match Dispatcher::builder(
            bot.clone(),
            entry()
                .branch(schema.clone())
                .branch(schema2.clone())
                .branch(schema3.clone()),
        )
        .build()
        .dispatch()
        .await
        {
            _ => {}
        }
    }

    Ok(())
}
