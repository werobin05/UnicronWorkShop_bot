use teloxide::prelude::*;
use teloxide::types::{Message, ParseMode};

pub async fn help(bot: Bot, msg: Message) -> ResponseResult<()> {
    let help_text = format!("{}", "
        <b>📖 Доступные команды:</b>\n\n
        🚀 <b>/start</b> – начало работы с ботом; \n
        🆘 <b>/help</b> – помощь и поддержка;\n
        🛠️ <b>/services</b> – список доступных услуг;\n
        📦 <b>/orders</b> – список ваших заказов;\n\n        
        💬 Если у вас есть вопросы по поддержке, просто напишите сообщение.
    ");

    bot.send_message(msg.chat.id, help_text)
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
