use std::rc::Rc;

use ratatui::{
    buffer::Buffer, layout::{Alignment, Rect}, prelude::*, style::{Color, Styled, Stylize}, widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap}
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
        // .margin(2)
        .constraints([
            Constraint::Percentage(18),
            Constraint::Percentage(18),
            Constraint::Percentage(18),
            Constraint::Percentage(18),
            Constraint::Percentage(18),
            Constraint::Min(0),
        ])
        .split(frame.area());

    let border = Block::default()
        .title("Main UI - Press 'd' for data, 'l' for log")
        .border_style(Style::default().fg(Color::Red))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    frame.render_widget(&border, frame.area());
    // Draw popup if enabled
    if app.widget_data.is_displayed(WidgetKind::Popup) {
        render_data_popup(app, frame, &chunks);
    }
    if app.widget_data.is_displayed(WidgetKind::Log) {
        render_log_popup(app, frame, &chunks);
    }
    if app.widget_data.is_displayed(WidgetKind::Inventory) {
        render_inventory_popup(app, frame, &chunks);
    }
    if app.widget_data.is_displayed(WidgetKind::Player) {
        render_player_popup(app, frame, &chunks);
    }
    if app.widget_data.is_displayed(WidgetKind::Shotgun) {
        render_shotgun_popup(app, frame, &chunks);
    }

}

fn render_data_popup(app: &App, frame: &mut Frame, chunks: &[Rect]) {
    let area = centered_rect(60, 30, frame.area());

    let popup_content = format!(
        "Data: {:?} Counter: {}", app.data, app.counter,
    );
    let mut data_popup = Paragraph::new(popup_content)
        .block(Block::default().title("Popup").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    if app.widget_data.is_focused(WidgetKind::Popup) {
        data_popup = data_popup.set_style(Style::default().fg(Color::LightRed))
    }
    frame.render_widget(Clear, area); // Clear background behind popup
    frame.render_widget(data_popup, chunks[0]);
}

fn render_log_popup(app: &App, frame: &mut Frame, chunks: &[Rect]) {
    let area = chunks[1];
    let log_content = app.log.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    let mut log_popup = Paragraph::new(log_content)
        .block(Block::default().title("Message Log").borders(Borders::ALL))
        .wrap(Wrap {trim: true})
        .scroll((app.log_scroll, 0));
    if app.widget_data.is_focused(WidgetKind::Log) {
        log_popup = log_popup.set_style(Style::default().fg(Color::LightRed))
    }

    frame.render_widget(Clear, area);
    frame.render_widget(log_popup, chunks[1]);
}

fn render_inventory_popup(app: &App, frame: &mut Frame, chunks: &[Rect]) {
    let area = chunks[2];
    let log_content = app.log.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    let mut log_popup = Paragraph::new(log_content)
        .block(Block::default().title("Message Log").borders(Borders::ALL))
        .wrap(Wrap {trim: true})
        .scroll((app.log_scroll, 0));
    if app.widget_data.is_focused(WidgetKind::Inventory) {
        log_popup = log_popup.set_style(Style::default().fg(Color::LightRed))
    }

    frame.render_widget(Clear, area);
    frame.render_widget(log_popup, chunks[2]);
}

fn render_player_popup(app: &App, frame: &mut Frame, chunks: &[Rect]) {
    let area = chunks[3];
    let log_content = app.log.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    let mut log_popup = Paragraph::new(log_content)
        .block(Block::default().title("Message Log").borders(Borders::ALL))
        .wrap(Wrap {trim: true})
        .scroll((app.log_scroll, 0));
    if app.widget_data.is_focused(WidgetKind::Player) {
        log_popup = log_popup.set_style(Style::default().fg(Color::LightRed))
    }

    frame.render_widget(Clear, area);
    frame.render_widget(log_popup, chunks[3]);
}

fn render_shotgun_popup(app: &App, frame: &mut Frame, chunks: &[Rect]) {
    let area = chunks[4];
    let log_content = app.log.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    let mut log_popup = Paragraph::new(log_content)
        .block(Block::default().title("Message Log").borders(Borders::ALL))
        .wrap(Wrap {trim: true})
        .scroll((app.log_scroll, 0));
    if app.widget_data.is_focused(WidgetKind::Shotgun) {
        log_popup = log_popup.set_style(Style::default().fg(Color::LightRed))
    }

    frame.render_widget(Clear, area);
    frame.render_widget(log_popup, chunks[4]);
}
