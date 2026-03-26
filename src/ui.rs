use ratatui::{
    buffer::Buffer, symbols, layout::{Alignment, Rect}, prelude::*, style::{Color, Styled, Stylize}, widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap, canvas::Canvas, canvas::Line}
};

//add svg crate
/* use svg::{Tree, NodeKind}; */

use crate::app::{ App };
use crate::{components::enums::ShotgunCycleView, ui_components::widget_data::{self, WidgetData, WidgetKind}};
use crate::components::match_data::MatchData;
use crate::components::player::Player;

pub const PLAYER_ART: &str = include_str!("assets/player_icon.txt");
//maybe do compile time solving
/* const PLAYER_ART_WIDTH: ;
const PLAYER_ART_HEIGHT: ; */

pub const SHOTGUN_ART: &str = include_str!("assets/shotgun.txt");
pub const BANG: &str = include_str!("assets/bang.txt");
pub const CLICK: &str = include_str!("assets/click.txt");
pub const SHELL: &str = include_str!("assets/shell.txt");

pub const ZAP: &str = r#"
"#;

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

pub fn render_ui(app: &App, frame: &mut Frame, widget_data: &WidgetData) -> Option<String> {
    //will eventually get rid of this
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

    //always on the bottom
    // render_shotgun_popup(frame, widget_data);

    //just add it to the render_stack
    /* render_player_popup(frame, &widget_data, &app.match_data, 0);
    render_player_popup(frame, &widget_data, &app.match_data, 1); */

    for kind in &widget_data.render_stack {
        let state = widget_data.get_state(*kind);
        if state.display {
            match kind {
                WidgetKind::Data => render_data_popup(app, frame, &widget_data),
                WidgetKind::Log => render_log_popup(app, frame, &widget_data),
                WidgetKind::Inventory => render_inventory_popup(app, frame, &widget_data, &chunks),
                WidgetKind::Player(i) => render_player_popup(frame, &widget_data, &app.match_data, *i),
                WidgetKind::Shotgun => render_shotgun_popup(frame, &widget_data),
                _ => return Some("shotgun is already displayed by default".to_string()),
            }
        }
    }
    return None
}

/* fn render_generic_popup(frame: &mut Frame, area: Rect, content: &str, focused: bool, color: Color) {
    let mut popup = Paragraph::new(content)
        .block(Block::default().title("Popup").borders(Borders::ALL))
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(color));

    if focused {
        popup = popup.set_style(Style::default().fg(Color::LightRed));
    }

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
} */

fn render_data_popup(app: &App, frame: &mut Frame, widget_data: &WidgetData) {

    let term_area = frame.area();
    let term_width = term_area.width;
    let term_height = term_area.height;

    let width = term_width * 50 / 100;
    let height = term_height * 30 / 100;
    let area = Rect {
        x: 1,
        y: 2,
        width,
        height,
    };

    let popup_content = format!(
        "Data: {:?} Counter: {}\nWindow: {:?}", app.data, app.match_data.round_count(), app.logger.get_window(),
    );

    let mut data_popup = Paragraph::new(popup_content)
        .block(Block::default().title("Data").borders(Borders::ALL))
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White));

    if widget_data.is_focused(WidgetKind::Data) {
        data_popup = data_popup.set_style(Style::default().fg(Color::LightRed));
    }

    frame.render_widget(Clear, area);
    frame.render_widget(data_popup, area);
}

fn render_log_popup(app: &App, frame: &mut Frame, widget_data: &WidgetData) {

    let area = frame.area();
    let width = (area.width as f32 * 0.33) as u16;
    let height = (area.height as f32 * 0.75) as u16;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let area = Rect {
        x,
        y,
        width,
        height,
    };

    let log_content = app.logger.get_window().iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    let mut log_popup = Paragraph::new(log_content)
        .block(Block::default().title("Message Log - use j k to navigate").borders(Borders::ALL))
        .wrap(Wrap {trim: true})
        .scroll((( app.logger.log_scroll as u16), 0));
    if widget_data.is_focused(WidgetKind::Log) {
        log_popup = log_popup.set_style(Style::default().fg(widget_data.get_color(&WidgetKind::Log).unwrap_or(Color::White)));
    }

    frame.render_widget(Clear, area);
    frame.render_widget(log_popup, area);
}


fn render_inventory_popup(app: &App, frame: &mut Frame, widget_data: &WidgetData, chunks: &[Rect]) {

    let area = chunks[2];
    let log_content = app.logger.log.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    let mut log_popup = Paragraph::new(log_content)
        .block(Block::default().title("Message Log").borders(Borders::ALL))
        .wrap(Wrap {trim: true})
        .scroll(((app.logger.log_scroll as u16), 0));
    if widget_data.is_focused(WidgetKind::Inventory) {
        log_popup = log_popup.set_style(Style::default().fg(widget_data.get_color(&WidgetKind::Inventory).unwrap_or(Color::White)))
    }

    frame.render_widget(Clear, area);
    frame.render_widget(log_popup, chunks[2]);
}

fn measure_art(art: &str) -> (u16, u16) {
    let height = art.lines().count() as u16;

    let width = art
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as u16;

    (width, height)
}

fn render_player_popup(frame: &mut Frame, widget_data: &WidgetData, match_data: &MatchData, i: usize) {
    let frame_area = frame.area();
    let player = match_data.players[i].clone();

    let (w, h) = measure_art(player.art);

    let x = frame_area.x + (frame_area.width * 5 / 10) - w / 2;

    let y = match i {
        0 => frame_area.y + 1,
        1 => frame_area.y + frame_area.height.saturating_sub(h + 1),
        _ => frame_area.y + 1,
    };

    let area = Rect {
        x,
        y,
        width: w,
        height: h,
    };

    let mut player_color = Color::White;

    if let Some(WidgetKind::Player(num)) = widget_data.get_focus() {
        if num == i {
            player_color = Color::LightRed;
        }
    }

    if match_data.player_turn == Some(i) {
        player_color = Color::Red;
    }

    let player_popup = Paragraph::new(player.art.to_string())
        .style(Style::default().fg(player_color))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(player.name.clone())
                .title_style(Style::default().fg(player_color))
                .borders(Borders::NONE),
        );

    frame.render_widget(Clear, area);
    frame.render_widget(player_popup, area);
}

//begin changing "popups" to not be such as shotgun and inventory
fn render_shotgun_popup(frame: &mut Frame, widget_data: &WidgetData) {

    let frame_area = frame.area();

    /* const DEFAULT_WIDTH: u16 = 68;
    const DEFAULT_HEIGHT: u16 = 10;

    let anchor_w = DEFAULT_WIDTH.min(frame.area().width);
    let anchor_h = DEFAULT_HEIGHT.min(frame.area().height);

    let anchor_x = frame_area.x + (frame_area.width - anchor_w) / 2;
    let anchor_y = frame_area.y + (frame_area.height - anchor_h) / 2; */

    match widget_data.shotgun_cycle_view {
        ShotgunCycleView::Shooting => {
            //find the correct values here
            let w = 105;
            let h = 10;
            let w = w.min(frame_area.width);
            let h = h.min(frame_area.height);

            let x = frame_area.x + (frame_area.width - w) / 2;
            let y = frame_area.y + (frame_area.height - h) / 2;

            let area = Rect { x, y, width: w, height: h };

            let shotgun_popup = Paragraph::new(BANG)
                .block(Block::default().borders(Borders::empty()));
            frame.render_widget(Clear, area);
            frame.render_widget(shotgun_popup, area);
        },
        ShotgunCycleView::Blanking => {
            let w = 90;
            let h = 10;
            let w = w.min(frame_area.width);
            let h = h.min(frame_area.height);

            let x = frame_area.x + (frame_area.width - w) / 2;
            let y = frame_area.y + (frame_area.height - h) / 2;

            let area = Rect { x, y, width: w, height: h };

            let shotgun_popup = Paragraph::new(CLICK)
                .block(Block::default().borders(Borders::empty()));
            frame.render_widget(Clear, area);
            frame.render_widget(shotgun_popup, area);
        },
        ShotgunCycleView::Reloading => {
            let w = 68;
            let h = 10;
            let w = w.min(frame_area.width);
            let h = h.min(frame_area.height);

            let x = frame_area.x + (frame_area.width - w) / 2;
            let y = frame_area.y + (frame_area.height - h) / 2;

            let area = Rect { x, y, width: w, height: h };

            let shotgun_popup = Paragraph::new(SHOTGUN_ART)
                .block(Block::default().borders(Borders::empty()));
            frame.render_widget(Clear, area);
            frame.render_widget(shotgun_popup, area);
        },
        _=> {
            let w = 68;
            let h = 10;
            let w = w.min(frame_area.width);
            let h = h.min(frame_area.height);

            let x = frame_area.x + (frame_area.width - w) / 2;
            let y = frame_area.y + (frame_area.height - h) / 2;

            let area = Rect { x, y, width: w, height: h };

            let shotgun_popup = Paragraph::new(SHOTGUN_ART)
                .block(Block::default().borders(Borders::empty()));
            frame.render_widget(Clear, area);
            frame.render_widget(shotgun_popup, area);
        }
    }
}

//want to make a popup to confirm things
fn render_confirm_popup(app: &App, frame: &mut Frame) {
    //three rects/ one big one and two small ones
    let frame_area = frame.area();
    let w = frame_area.width;
    let h = frame_area.height;
}
