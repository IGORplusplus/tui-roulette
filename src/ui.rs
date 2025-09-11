use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget, Borders, Wrap, Clear},
    prelude::*,
};

use crate::uihelp::widget_data::{WidgetKind, WidgetData};

use crate::app::{ App };

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

pub fn render_ui(app: &App, frame: &mut Frame){
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());
    let main_block = Block::default()
        .title("Main UI - Press 'p' for popup, 'l' for log")
        .border_style(Style::default().fg(Color::Red))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);
    frame.render_widget(main_block, frame.area());
    // Draw popup if enabled
    if app.widget_data.is_displayed(WidgetKind::Popup) {
        let area = centered_rect(60, 20, frame.area());

        let popup_text = format!(
            "Data: {:?} Counter: {}", app.data, app.counter,
        );
        let popup = Paragraph::new(popup_text)
            .block(Block::default().title("Popup").borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        frame.render_widget(Clear, area); // Clear background behind popup
        frame.render_widget(popup, chunks[0]);
    }
    if app.widget_data.is_displayed(WidgetKind::Log) {
        let area = centered_rect(60, 20, frame.area());
        let log_content = app.log.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
        let log_popup = Paragraph::new(log_content)
            .block(Block::default().title("Message Log").borders(Borders::ALL))
            .wrap(Wrap {trim: true})
            .scroll((app.log_scroll, 0));

        frame.render_widget(Clear, area);
        frame.render_widget(log_popup, area);
    }
}
