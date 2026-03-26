use crate::app::App;

pub mod app;
pub mod components;
pub mod data;
pub mod event;
pub mod ui;
pub mod ui_components;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
