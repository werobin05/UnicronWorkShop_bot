use teloxide::utils::command::BotCommands;

pub mod help;
pub mod start;
pub mod orders;
pub mod services;
pub mod settings;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description ="list commands")]
pub enum Command {
    #[command(description = "Start work")]
    Start,
    #[command(description = "F.A.Q")]
    Help,
    #[command(description = "market order")]
    Services,
    #[command(description = "My orders")]
    Orders,
    #[command(description = "Settings bot")]
    Settings,
}