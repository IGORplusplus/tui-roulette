use std::collections::VecDeque;

use crate::components::enums::ReloadAmount;
use crate::uihelp::widget_data::{WidgetData, WidgetKind};
use crate::ui;

use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    /* style::{ Color, Style, Stylize },
    widgets::{Block, Borders, Clear, Paragraph, Wrap, BorderType}, */
    Frame,
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers, self},
};

use crate::data::Data;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,
    /// game data
    pub data: Data,
    ///holds the information of the widgets
    pub widget_data: WidgetData,
    /// log for popup texts
    pub log: VecDeque<String>,
    ///Where is the log scrolled to
    pub log_scroll: u16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            data: Data::new(),
            log: VecDeque::new(),
            log_scroll: 0,
            widget_data: WidgetData::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send_log(&mut self, message: Option<String>) {
        let max_size: usize = 10;
        if self.log.len() > max_size {
            self.log.pop_front();
        }
        self.log.push_back(message.unwrap());
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render_ui(frame))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::Reload(amount) => {
                        self.data.shotgun.load_random_shells(amount.as_usize());
                    },
                    AppEvent::Shoot => {
                        if let Some(msg) = self.data.shotgun.shoot() {
                            self.send_log(Some(msg));
                        }
                    },
                    AppEvent::Popup => {
                        if self.widget_data.is_displayed(WidgetKind::Popup) {
                            self.widget_data.set_widget(WidgetKind::Popup, false, false)
                        } else {
                            self.widget_data.set_widget(WidgetKind::Popup, true, true)
                        }
                    },
                    AppEvent::Log => {
                        if self.widget_data.is_displayed(WidgetKind::Log) {
                            self.widget_data.set_widget(WidgetKind::Log, false, false)
                        } else {
                            self.widget_data.set_widget(WidgetKind::Log, true, true)
                        }
                    },
                    AppEvent::ScrollUp => {
                        if self.log_scroll > 0 {
                            self.log_scroll -= 1;
                        }
                    },
                    AppEvent::ScrollDown => {
                        self.log_scroll += 1;
                    },
                    AppEvent::ChangeFocus => {
                        self.widget_data.focus_next();
                    },
                    AppEvent::ChangeFocusBack => {
                        self.widget_data.focus_prev();
                    },
                    _ => {
                        self.send_log(Some(String::from("Failure to catch event")));
                    },
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Char('p' | 'P') => self.events.send(AppEvent::Popup),
            KeyCode::Char('l' | 'L') => self.events.send(AppEvent::Log),
            KeyCode::Char('k') if self.widget_data.is_focused(WidgetKind::Log) => self.events.send(AppEvent::ScrollUp),
            KeyCode::Char('j') if self.widget_data.is_focused(WidgetKind::Log) => self.events.send(AppEvent::ScrollDown),
            KeyCode::Tab if key_event.modifiers == KeyModifiers::CONTROL => self.events.send(AppEvent::ChangeFocusBack),
            KeyCode::Tab => self.events.send(AppEvent::ChangeFocus),
            KeyCode::Char('r' | 'R') => {
                match self.data.round_count {
                    1 => self.events.send(AppEvent::Reload(ReloadAmount::One)),
                    2 => self.events.send(AppEvent::Reload(ReloadAmount::Two)),
                    3 => self.events.send(AppEvent::Reload(ReloadAmount::Three)),
                    4 => self.events.send(AppEvent::Reload(ReloadAmount::Four)),
                    5 => self.events.send(AppEvent::Reload(ReloadAmount::Five)),
                    _ => self.events.send(AppEvent::Reload(ReloadAmount::Five)),
                }
            }
            KeyCode::Char(' ') => self.events.send(AppEvent::Shoot),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    fn render_ui(&self, frame: &mut Frame){
        ui::render_ui(self, frame);
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

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
