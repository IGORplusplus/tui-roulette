use std::rc::Rc;

use ratatui::{
    buffer::Buffer, symbols, layout::{Alignment, Rect}, prelude::*, style::{Color, Styled, Stylize}, widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap, canvas::Canvas, canvas::Line}
};

//add svg crate
/* use svg::{Tree, NodeKind}; */

use crate::uihelp::widget_data::{WidgetKind, WidgetData};
use crate::app::{ App };

const PLAYER_ART: &str = r#"
 (\_/)
 ( •_•)
/>🍪
"#;

pub const SHOTGUN_ART: &str = r#"
 ,______________________________________
|_________________,----------._ [____]  ""-,__  __....-----=====
               (_(||||||||||||)___________/   ""                |
                  `----------'        [ ))"-,                   |
                                       ""    `,  _,--....___    |
                                               `/           """"
"#;

pub const BANG: &str = r#"
########     ###    ##    ##  ######   ,______________________________________
##     ##   ## ##   ###   ## ##    ## |_________________,----------._ [____]  ""-,__  __....-----=====
##     ##  ##   ##  ####  ## ##                      (_(||||||||||||)___________/   ""                |
########  ##     ## ## ## ## ##   ####                  `----------'        [ ))"-,                   |
##     ## ######### ##  #### ##    ##                                        ""    `,  _,--....___    |
##     ## ##     ## ##   ### ##    ##                                                `/           """"
########  ##     ## ##    ##  ######
"#;

pub const CLICK: &str = r#"
     |    o     |     ,______________________________________
,---.|    .,---.|__/ |_________________,----------._ [____]  ""-,__  __....-----=====
|    |    ||    |  \                (_(||||||||||||)___________/   ""                |
`---'`---'``---'`   `                  `----------'        [ ))"-,                   |
                                                            ""    `,  _,--....___    |
                                                                    `/           """"
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

pub fn render_ui(app: &App, frame: &mut Frame){
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

    // does it in order of the "stack"
    for kind in &app.widget_data.render_stack {
        let state = app.widget_data.get_state(*kind);
        if state.display {
            match kind {
                WidgetKind::Data => render_data_popup(app, frame),
                WidgetKind::Log => render_log_popup(app, frame),
                WidgetKind::Inventory => render_inventory_popup(app, frame, &chunks),
                WidgetKind::Player => render_player_popup(app, frame),
                WidgetKind::Shotgun => render_shotgun_popup(app, frame),
            }
        }
    }
}

fn render_data_popup(app: &App, frame: &mut Frame) {
    let term_area = frame.area();
    let term_width = term_area.width;
    let term_height = term_area.height;

    let width = term_width * 30 / 100;
    let height = term_height * 30 / 100;
    let area = Rect {
        x: 1,
        y: 2,
        width,
        height,
    };

    let popup_content = format!(
        "Data: {:?} Counter: {}", app.data, app.counter,
    );

    let mut data_popup = Paragraph::new(popup_content)
        .block(Block::default().title("Popup").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    if app.widget_data.is_focused(WidgetKind::Data) {
        data_popup = data_popup.set_style(Style::default().fg(Color::LightRed))
    }
    frame.render_widget(Clear, area); // Clear background behind popup
    frame.render_widget(data_popup, area);
}


fn render_log_popup(app: &App, frame: &mut Frame) {
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

    let log_content = app.log.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    let mut log_popup = Paragraph::new(log_content)
        .block(Block::default().title("Message Log - use j k to navigate").borders(Borders::ALL))
        .wrap(Wrap {trim: true})
        .scroll((app.log_scroll, 0));
    if app.widget_data.is_focused(WidgetKind::Log) {
        log_popup = log_popup.set_style(Style::default().fg(Color::LightRed));
    }

    frame.render_widget(Clear, area);
    frame.render_widget(log_popup, area);
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

fn render_player_popup(app: &App, frame: &mut Frame) {

    let area = Rect {
        x: 10,
        y: 5,
        width: 10,
        height: 10,
    };

    // The "icon" — can be emoji, unicode, ASCII art, etc.

    let mut player_popup = Paragraph::new(PLAYER_ART)
        .block(Block::default().title("Popup").borders(Borders::NONE));
    if app.widget_data.is_focused(WidgetKind::Data) {
        player_popup = player_popup.set_style(Style::default().fg(Color::LightRed))
    }

    frame.render_widget(Clear, area);
    frame.render_widget(player_popup, area);
}

//begin changing "popups" to not be such as shotgun and inventory
fn render_shotgun_popup(app: &App, frame: &mut Frame) {
    let frame_area = frame.area();
    let w = 68;
    let h = 10;
    let w = w.min(frame_area.width);
    let h = h.min(frame_area.height);

    let x = frame_area.x + (frame_area.width - w) / 2;
    let y = frame_area.y + (frame_area.height - h) / 2;

    let area = Rect { x, y, width: w, height: h };
    let mut shotgun_popup = Paragraph::new(SHOTGUN_ART)
        .block(Block::default().borders(Borders::empty()));

    if app.widget_data.is_focused(WidgetKind::Shotgun) {
        shotgun_popup = shotgun_popup.set_style(Style::default().fg(Color::LightRed))
    }

    frame.render_widget(Clear, area);
    frame.render_widget(shotgun_popup, area);
}

fn render_confirm_popup(app: &App, frame: &mut Frame) {
    //three rects/ one big one and two small ones
    let frame_area = frame.area();
    let w = frame_area.width;
    let h = frame_area.height;
}
