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
use std::io;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    //Is the popup showing?
    pub bool_popup: bool,
    //Is the log showing?
    pub bool_log: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,
    /// game data
    pub data: Data,
    /// log for popup texts
    pub log: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            bool_popup: false,
            bool_log: false,
            counter: 0,
            events: EventHandler::new(),
            data: Data::new(),
            log: Vec::new(),
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
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
                    AppEvent::Popup => self.show_popup(),
                    AppEvent::Log => self.show_log(),
                    AppEvent::Reload => {
                        /* let mut num_shells = String::new();
                        io::stdin().read_line(&mut num_shells)
                            .expect("Failed to read number of shells");
                        let num_shells: usize = num_shells
                            .trim()
                            .parse()
                            .expect("was not a usize number"); */
                        self.data.shotgun.load_random_shells(8);
                    },
                    AppEvent::Shoot => {
                        if let Some(msg) = self.data.shotgun.shoot() {
                            self.log.push(msg);
                        }
                    }
                    AppEvent::Quit => self.quit(),
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
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            KeyCode::Char('p' | 'P') => self.events.send(AppEvent::Popup),
            KeyCode::Char('r' | 'R') => self.events.send(AppEvent::Reload),
            KeyCode::Char('l' | 'L') => self.events.send(AppEvent::Log),
            KeyCode::Char(' ') => self.events.send(AppEvent::Shoot),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
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
        if self.bool_log{
            let area = centered_rect(60, 20, frame.area());
            let log_content = self.log.join("\n");
            let log_popup = Paragraph::new(log_content)
                .block(Block::default().title("Message Log").borders(Borders::ALL))
                .wrap(Wrap {trim: true});

            frame.render_widget(Clear, area);
            frame.render_widget(log_popup, area);
        }
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
