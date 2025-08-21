use std::env;
use teloxide::{Bot};
use teloxide::prelude::Request;
use rust_decimal::prelude::ToPrimitive;
use teloxide::types::{ChatId, LabeledPrice};
use teloxide::payloads::SendInvoiceSetters;
use teloxide::prelude::{Requester, ResponseResult};


use crate::models::Services;

pub async fn send_invoice_service(bot: &Bot, chat_id: i64, service: Services) -> ResponseResult<()> {
    let provider_token = env::var("PAYMENT_TOKEN")
        .expect("PAYMENT_TOKEN must be set in .env and совпадать с токеном в BotFather");

    let amount: u32 = (service.price.to_f64().unwrap_or(0.0) * 100.0).round() as u32;
    let prices = vec![
        LabeledPrice {
            label: service.name_service.clone(),
            amount,
        },
    ];

    let payload = format!("service_{}_user_{}", service.service_id, chat_id);

    bot.send_invoice(
        ChatId(chat_id),
        format!("Payments service: {}", service.name_service),
        "Payments service",
        payload,
        provider_token,
        prices.clone(),
    )
    .currency("USD")
    .send()
    .await?;
    Ok(())
}
