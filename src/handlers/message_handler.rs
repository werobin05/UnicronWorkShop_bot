use sqlx::PgPool;
use teloxide::{prelude::*, utils::command::BotCommands};
use crate::commands::{settings::settings, help::help, orders::orders, services::services, start::start, Command};

pub async fn message_handler(bot: Bot, msg: Message, pool: PgPool) -> ResponseResult<()> {

    if let Some(text) = msg.text() {
        match Command::parse(text, "unicron_workshop_bot") {
            Ok(Command::Start) => start(bot, msg, pool).await,
            Ok(Command::Help) => help(bot, msg).await,
            Ok(Command::Services) => services(bot, msg).await,
            Ok(Command::Orders) => orders(bot, msg, pool).await,
            Ok(Command::Settings) => settings(bot, msg).await,
            Err(_) => {
                 match text {
                    "📦 Услуги" => services(bot, msg).await?,
                    "❓ F.A.Q" => help(bot, msg).await?,
                    "🗂️ Заказы" => orders(bot, msg, pool).await?,
                    "⚙️ Настройки" => settings(bot, msg).await?,
                    "📩 Написать в поддержку" => {
                        bot.send_message(msg.chat.id, "Напишите нам на почту support@example.com").await?;
                    }
                    _ => {
                        bot.send_message(msg.chat.id, "Неизвестная команда, нажмите /help для просмотра всех доступных команд").await?;
                    }
                }
                Ok(())
            }
        }
    } else {
        Ok(())
 }

}
