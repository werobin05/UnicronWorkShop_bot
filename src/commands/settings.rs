
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};


pub async fn settings(bot: Bot, msg: Message) -> ResponseResult<()> {
    let settings_keyboard = InlineKeyboardMarkup::new(vec! [
        vec! [
            InlineKeyboardButton::callback("choose_lang", "choose_lang"),
        ],
        vec! [
            InlineKeyboardButton::callback("method_pay", "method_pay"), 
        ],
        vec! [
            InlineKeyboardButton::callback("added_email", "added_email"),
        ], 
        vec! [
            InlineKeyboardButton::callback("delete_account", "delete_account"),
        ],
        vec! [
            InlineKeyboardButton::callback("go_back", "back_menu"),
            // InlineKeyboardButton::callback(t("go_next"), "next_menu")
        ]
    ]);

    bot.send_message(msg.chat.id, format!("{}", "title_setting"))
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .reply_markup(settings_keyboard)
    .await?;
    Ok(())
}