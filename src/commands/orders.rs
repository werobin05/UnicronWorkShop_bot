use sqlx::PgPool;
use teloxide::prelude::*;
use crate::models::UserOrderInfo;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::repos::{order_repo::fetch_orders, user_repo::get_user_by_username};


const PER_PAGE: i32 = 5;

pub async fn orders(bot: Bot, msg: Message, pool: PgPool) -> ResponseResult<()> {
    let username = msg
        .from
        .as_ref()
        .and_then(|u| u.username.as_deref())
        .unwrap_or("unknown");

    let Some(user) = get_user_by_username(&pool, username).await.unwrap_or_else(|e| {
        log::error!("error get username: {:?}", e);
    None
    }) else {
        bot.send_message(msg.chat.id, format!("{}", "no_reg_system"))
            .await?;
        return Ok(());
    };
    
    let page = 1;
    let orders: Vec<UserOrderInfo> = fetch_orders(&pool, user.user_id, page, PER_PAGE as usize)
        .await
        .unwrap_or_default();

    if orders.is_empty() {
        bot.send_message(msg.chat.id, format!("{}", "empty_order"))
            .await?;
        return Ok(());
    }

    let mut text_table = String::from(format!("{}\n\n", "empty_order"));
    text_table.push_str(&format!("{}\n", "row_name_order"));
    text_table.push_str("|-|-|-|-|-|\n");

    for (i, order) in orders.iter().enumerate() {
        let date_str = order
            .created_at
            .map(|d| d.format("%d.%m.%Y").to_string())
            .unwrap_or_else(|| "—".to_string());

        let row = format!(
            "|{}|{}|{}|{:.2}|{}|\n",
            i + 1 + (page - 1) * PER_PAGE as usize,
            order.name_service,
            order.code,
            order.price,
            date_str
        );

        text_table.push_str(&row);
    }

    let nav_buttons = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("go_back", "orders_prev"),
        InlineKeyboardButton::callback("go_next", "orders_next"),
    ]]);

    bot.send_message(msg.chat.id, text_table)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(nav_buttons)
        .await?;

    Ok(())
}
