//std library
use std::collections::VecDeque;

//ratatui
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    /* style::{ Color, Style, Stylize },
    widgets::{Block, Borders, Clear, Paragraph, Wrap, BorderType}, */
    Frame,
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseButton, MouseEventKind, self},
};

//crossterm
use crossterm::{style::Color, terminal::{disable_raw_mode, enable_raw_mode}};
use crossterm::event::EnableMouseCapture;

//user made ones
use crate::{data::Data, ui_components::widget_data};
use crate::components::enums::ReloadAmount;
use crate::components::match_data::*;
use crate::ui_components::widget_data::{WidgetData, WidgetKind};
use crate::event::{AppEvent, Event, EventHandler};
use crate::ui;
use crate::ui_components::logger::Logger;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// game data
    pub data: Data,
    /// match data
    pub match_data: MatchData,
    ///holds the information of the widgets
    pub widget_data: WidgetData,
    /// logger will replace log, and it will automatically size to the correct screen size
    pub logger: Logger,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            data: Data::new(),
            match_data: MatchData::new(),
            widget_data: WidgetData::new(),
            logger: Logger::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send_log(&mut self, message: Option<String>) {
        if let Some(msg) = message {
            let max_size: usize = 1000;
            if self.logger.log.len() >= max_size {
                self.logger.log.pop_front();
            }
            self.logger.log.push_back(msg)
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

        while self.running {
            terminal.draw(|frame| {

                //changing logger's capacity
                let area = frame.area();
                let max_window_lines = ( area.height as f32 / 1.45 ) as usize;
                self.logger.set_window_size(max_window_lines);
                self.logger.update_window();
                self.render_ui(frame)})?;

            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    crossterm::event::Event::Mouse(mouse_event) => self.handle_mouse_events(mouse_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::Reload(amount) => {
                        self.data.shotgun.load_random_shells(amount.as_usize());
                    },
                    AppEvent::Shoot => {
                        if let Some(msg) = self.data.shotgun.shoot() {

                            //not a very robust solution but it works for now
                            if msg.contains("Last") {
                                self.match_data.next_round();
                            }
                            self.logger.send_log(Some(msg));
                            //if I shoot it becomes the focus
                            // self.widget_data.display_widget(WidgetKind::Shotgun, true);
                        }
                    },

                    AppEvent::ShowPopup(kind) => {
                        let kind = match kind {
                            Some(_) => kind.unwrap(),
                            _ => continue,
                        };

                        self.widget_data.display_widget(kind, true);
                        self.widget_data.render_stack.push(kind);
                    },

                    //kind is Option<WidgetKind>
                    AppEvent::HidePopup(kind) => {
                        //widget logic
                        let kind_copy = kind;
                        match kind_copy {
                            Some(k) => self.send_log(Some(format!("hiding {:?}", k).to_owned())),
                            _ => continue,
                        }
                        if self.widget_data.is_displayed(kind_copy.unwrap()) {
                            self.widget_data.hide_widget(kind.unwrap());
                        }
                        self.widget_data.render_stack.retain(|k| *k != kind.unwrap());
                        if !self.widget_data.render_stack.is_empty() {
                            self.widget_data.focus_next();
                        }
                        
                    },

                    AppEvent::FocusShotgun => {
                        self.widget_data.toggle_focus(WidgetKind::Shotgun);
                    },

                    AppEvent::ScrollUp => {
                        /* if self.logger.log_scroll > 0 {
                            self.logger.scroll_up();
                        } */
                    },
                    AppEvent::ScrollDown => {
/*                         self.logger.scroll_down(); */
                    },
                    AppEvent::ChangeFocus => {
                        self.widget_data.focus_next();
                    },
                    AppEvent::ChangeFocusBack => {
                        self.widget_data.focus_prev();
                    },
                    _ => {
                        self.logger.send_log(Some(String::from("Failure to catch event")));
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
            KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            },

            KeyCode::Char('d' | 'D') => {
                self.events.send(AppEvent::ShowPopup(Some(WidgetKind::Data)))
            },
            KeyCode::Char('l' | 'L') => {
                self.events.send(AppEvent::ShowPopup(Some(WidgetKind::Log)));
            },

            //KeyCode::Char('i' | 'I') => self.events.send(AppEvent::ShowInventory),
            KeyCode::Char('p' | 'P') => self.events.send(AppEvent::ShowPopup(Some(WidgetKind::Player))),
            // KeyCode::Char('s' | 'S') => self.events.send(AppEvent::FocusShotgun),
            KeyCode::Char('x' | 'X') => self.events.send(AppEvent::HidePopup(self.widget_data.get_focus())),
            KeyCode::Char('k') if self.widget_data.is_focused(WidgetKind::Log) => self.events.send(AppEvent::ScrollUp),
            KeyCode::Char('j') if self.widget_data.is_focused(WidgetKind::Log) => self.events.send(AppEvent::ScrollDown),
            KeyCode::Tab if key_event.modifiers == KeyModifiers::CONTROL => self.events.send(AppEvent::ChangeFocusBack),
            KeyCode::Tab => self.events.send(AppEvent::ChangeFocus),
            KeyCode::Char('r' | 'R') => {
                match self.match_data.round_count() {
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

    pub fn handle_mouse_events(&mut self, mouse_event: MouseEvent) -> color_eyre::Result<()> {
        match mouse_event.kind {
            MouseEventKind::ScrollUp => {
                self.logger.send_log(Some("scrolling up".to_string()));
                self.events.send(AppEvent::ScrollUp)
            },
            MouseEventKind::Drag(mouse_button) => {
                match mouse_button {
                    MouseButton::Left => {
                        self.logger.send_log(Some("left dragging".to_string()));
                    },
                    _ => {
                        self.logger.send_log(Some("some other dragging".to_string()));
                    },
                }
            },
            _ => {
            }
        }
        Ok(())
    }

    fn render_ui(&mut self, frame: &mut Frame){
        let log: Option<String> = ui::render_ui(self, frame);
        self.logger.send_log(log);
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
