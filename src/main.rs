use crate::app::App;

pub mod event;
pub mod ui;
pub mod components;
pub mod data;
pub mod app;

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    let result = App::new().run().await;
    disable_raw_mode()?;
    result
}
