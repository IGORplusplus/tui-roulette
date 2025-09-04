use std::collections::VecDeque;

use crate::components::enums::ReloadAmount;

use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{ Color, Style, Stylize },
    widgets::{Block, Borders, Clear, Paragraph, Wrap, BorderType},
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
    //Is the popup showing?
    pub bool_popup: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,
    /// game data
    pub data: Data,
    /// log for popup texts
    pub log: VecDeque<String>,
    //Is the log showing?
    pub bool_log: bool,
    ///Where is the log scrolled to
    pub log_scroll: u16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            bool_popup: false,
            counter: 0,
            events: EventHandler::new(),
            data: Data::new(),
            log: VecDeque::new(),
            bool_log: false,
            log_scroll: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
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
                    AppEvent::Popup => self.show_popup(),
                    AppEvent::Log => self.show_log(),
                    AppEvent::ScrollUp => {
                        if self.log_scroll > 0 {
                            self.log_scroll -= 1;
                        }
                    }
                    AppEvent::ScrollDown => {
                        self.log_scroll += 1;
                    }
                    AppEvent::Reload(amount) => {
                        self.data.shotgun.load_random_shells(amount.as_usize());
                    },
                    AppEvent::Shoot => {
                        if let Some(msg) = self.data.shotgun.shoot() {
                            let max_size: usize = 10;
                            if self.log.len() > max_size {
                                self.log.pop_front();
                            }
                            self.log.push_back(msg);
                        }
                    }
                    AppEvent::Quit => self.quit(),
                },
                _ => {
                    self.log.push_back("Failure to catch app event!".to_string());
                }
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
            KeyCode::Char('j') => self.events.send(AppEvent::ScrollUp),
            KeyCode::Char('k') => self.events.send(AppEvent::ScrollDown),
            KeyCode::Tab => self.events.send(AppEvent::ForwardBlock),
            KeyCode::Tab if key_events.modifiers == KeyModifiers::CONTROL => self.events.send(AppEvent::BackBlock),
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

    fn draw_log(&self, frame: &mut Frame) {
        if self.bool_log{
            let area = centered_rect(60, 20, frame.area());
            let log_content = self.log.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
            let log_popup = Paragraph::new(log_content)
                .block(Block::default().title("Message Log").borders(Borders::ALL))
                .wrap(Wrap {trim: true})
                .scroll((self.log_scroll, 0));

            frame.render_widget(Clear, area);
            frame.render_widget(log_popup, area);
        }
    }

    fn render_ui(&self, frame: &mut Frame){
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
        if self.bool_popup {
            let area = centered_rect(60, 20, frame.area());

            let popup_text = format!(
                "Data: {:?} Counter: {}", self.data, self.counter,
            );
            let popup = Paragraph::new(popup_text)
                .block(Block::default().title("Popup").borders(Borders::ALL))
                .wrap(Wrap { trim: true });

            frame.render_widget(Clear, area); // Clear background behind popup
            frame.render_widget(popup, chunks[0]);
        }
        self.draw_log(frame);
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

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub fn show_popup(&mut self) {
        self.bool_popup = !self.bool_popup;
    }

    pub fn show_log(&mut self) {
        self.bool_log = !self.bool_log;
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
