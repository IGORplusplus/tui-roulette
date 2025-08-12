use crate::app::App;

pub mod event;
pub mod ui;
pub mod components;
pub mod data;
pub mod app;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
