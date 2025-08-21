use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn services(bot: Bot, msg: Message) -> ResponseResult<()> {

    let inline_keyboard = InlineKeyboardMarkup::new(vec! [
        vec! [
            InlineKeyboardButton::callback("🎨 Дизайн", "service_design"),
            InlineKeyboardButton::callback("👨‍💻 Разработка", "service_dev"),
        ],
        vec! [
            InlineKeyboardButton::callback("📊 Маркетинг", "service_marketing"),
            InlineKeyboardButton::callback("📑 Копирайтинг", "service_cooperate"),
        ],
        vec! [
            InlineKeyboardButton::callback("👨‍💻 Присоединиться к команде", "join_command"),
        ],
        vec! [
            InlineKeyboardButton::callback("⬅️ Назад", "back_menu"),
        ]
    ]);
    bot.send_message(msg.chat.id, format!("{}", "🔧 *Выберите интересующую вас услугу:*"))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(inline_keyboard)
        .await?;
    Ok(())
}
