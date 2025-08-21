use sqlx::PgPool;
use teloxide::prelude::*;
use crate::services::send_invoice_service;
use crate::ui::keyboard::{keyboard_from_services};
use crate::repos::service_repo::fetch_by_category;
use crate::{models::ServiceCode, repos::service_repo::fetch_service_by_id};
use teloxide::types::{CallbackQuery, InlineKeyboardMarkup, InlineKeyboardButton, ParseMode};


pub async fn callback_handler(
    bot: Bot, q: CallbackQuery, pool: PgPool) -> ResponseResult<()> {
    let data = q.data.clone().unwrap_or_else(|| "Данных нету".into());
    
    let username = q.from.username.as_deref().unwrap_or("Неизвестный");


    let title = "Выберите нужный вам вид услуги 📦";
    bot.answer_callback_query(q.id.clone())
        .text(format!("{} {}", data, &title))
        .await?;

    if let Some(ref msg) = q.message {
        let chat = msg.chat();

        match data.as_str() {
            "service_design" | "service_dev" | "service_marketing" => {
                let category_code = match data.as_str() {
                    "service_design" => ServiceCode::Design,
                    "service_dev" => ServiceCode::Dev,
                    "service_marketing" => ServiceCode::Marketing,
                    _ => {
                        bot.send_message(chat.id, "Неизвестна категория")
                            .await?;
                        return Ok(());
                    }
                };
                match fetch_by_category(&pool, category_code).await {
                    Ok(services) => {
                        let keyboard: InlineKeyboardMarkup = keyboard_from_services(&services);
                        bot.send_message(chat.id, "Выберите пакет")
                            .reply_markup(keyboard)
                            .await?;
                    }
                    Err(e) => {
                        eprintln!("❌ Error when receiving services: {:?}", e);
                        bot.send_message(chat.id, "Не удается загрузить список услуг.")
                            .await?;
                    }
                }
            }

            "back_to_services" => {
                bot.delete_message(msg.chat().id, msg.id()).await?;
            }

            "back_menu" => {
                bot.delete_message(msg.chat().id, msg.id()).await?;
            }
            
            "join_command" => {
                
                let nav_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::callback("fill_form", "fill_form"),
                    ],
                    vec![
                        InlineKeyboardButton::callback("go_back_view", "back_to_services"),
                    ],
                ]);

                bot.send_message(msg.chat().id, format!(
                    "👋 Привет, @{}!\n\n\
                    Решили присоединиться к нашей команде? — <b>Команда Unicron</b> 🔰\n\n\
                    Чтобы стать частью команды, нужно пройти короткое собеседование, \
                    показать свои навыки и работы.\n\n\
                    <b>Что вас ждёт:</b>\n\
                    📍 Весёлая и дружная команда\n\
                    📍 Задачи по вашему профилю\n\
                    📍 Возможности для роста и развития\n\n\
                    Мы уже ждём вас в нашей команде!",
                    username
                ))
                .reply_markup(nav_keyboard)
                .parse_mode(ParseMode::Html)
                .await?;
            }

            other if other.starts_with("service_") => {
                let id_str = other.trim_start_matches("service_");
                if let Ok(service_id) = id_str.parse::<i32>() {
                    if let Ok(service) = fetch_service_by_id(&pool, service_id).await {
                        send_invoice_service(&bot, chat.id.0, service).await?;
                    } else {
                        bot.send_message(chat.id, format!("{}", "Услуга не найдена")).await?;
                    }
                } else {
                    bot.send_message(chat.id, format!("{}", "Не существует такой услуги")).await?;
                }
            }

            _ => {
                bot.send_message(chat.id, format!("{}", "Вы выбрали неизвестную услугу"))
                    .await?;
            }
        }
    }

    Ok(())
}
