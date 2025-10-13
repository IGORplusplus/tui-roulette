//std library
use std::collections::VecDeque;

use crate::components::enums::ReloadAmount;
use crate::uihelp::widget_data::{WidgetData, WidgetKind};
use crate::ui;

use crate::event::{AppEvent, Event, EventHandler};
use crossterm::event::EnableMouseCapture;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    /* style::{ Color, Style, Stylize },
    widgets::{Block, Borders, Clear, Paragraph, Wrap, BorderType}, */
    Frame,
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseButton, MouseEventKind, self},
};

//crossterm
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

//user made ones
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
        if let Some(msg) = message {
            let max_size: usize = 1000;
            if self.log.len() >= max_size {
                self.log.pop_front();
            }
            self.log.push_back(msg)
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

        while self.running {
            terminal.draw(|frame| self.render_ui(frame))?;
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
                            self.send_log(Some(msg));
                        }
                    },
                    AppEvent::ShowData => {
                        if self.widget_data.is_displayed(WidgetKind::Data) {
                            self.widget_data.set_widget(WidgetKind::Data, false, false);
                            self.widget_data
                                .render_stack
                                .retain(|k| *k != WidgetKind::Data);
                            self.widget_data.focus_next();
                            if let Some(first) = self.widget_data.render_stack.first().cloned() {
                                self.widget_data.kind_focus(&first);
                            }
                            /* if let Some(first) = self.widget_data.render_stack.iter().find(|w| **w != WidgetKind::Shotgun).cloned() {
                                self.widget_data.kind_focus(&first);
                            } */
                        } else {
                            self.widget_data.set_widget(WidgetKind::Data, true, true);
                            //this is to get the rendering in the right order
                            self.widget_data.render_stack.push(WidgetKind::Data)
                        }
                    },
                    AppEvent::ShowLog => {
                        if self.widget_data.is_displayed(WidgetKind::Log) {
                            self.widget_data.set_widget(WidgetKind::Log, false, false);
                            self.widget_data
                                .render_stack
                                .retain(|k| *k != WidgetKind::Log);
                            if let Some(first) = self.widget_data.render_stack.first().cloned() {
                                self.widget_data.kind_focus(&first);
                            }
                        } else {
                            self.widget_data.set_widget(WidgetKind::Log, true, true);
                            self.widget_data.render_stack.push(WidgetKind::Log)
                        }
                    },
                    AppEvent::ShowInventory => {
                        if self.widget_data.is_displayed(WidgetKind::Inventory) {
                            self.widget_data.set_widget(WidgetKind::Inventory, false, false);
                            self.widget_data
                                .render_stack
                                .retain(|k| *k != WidgetKind::Inventory);
                            if let Some(first) = self.widget_data.render_stack.first().cloned() {
                                self.widget_data.kind_focus(&first);
                            }
                        } else {
                            self.widget_data.set_widget(WidgetKind::Inventory, true, true);
                            self.widget_data.render_stack.push(WidgetKind::Inventory)
                        }
                    },
                    AppEvent::ShowPlayer => {
                        if self.widget_data.is_displayed(WidgetKind::Player) {
                            self.widget_data.set_widget(WidgetKind::Player, false, false);
                            self.widget_data
                                .render_stack
                                .retain(|k| *k != WidgetKind::Player);
                            if let Some(first) = self.widget_data.render_stack.first().cloned() {
                                self.widget_data.kind_focus(&first);
                            }
                        } else {
                            self.widget_data.set_widget(WidgetKind::Player, true, true);
                            self.widget_data.render_stack.push(WidgetKind::Player)
                        }
                    },
                    AppEvent::FocusShotgun => {
                        self.widget_data.toggle_focus(WidgetKind::Shotgun);
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
            KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Char('d' | 'D') => self.events.send(AppEvent::ShowData),
            KeyCode::Char('l' | 'L') => self.events.send(AppEvent::ShowLog),
            //KeyCode::Char('i' | 'I') => self.events.send(AppEvent::ShowInventory),
            KeyCode::Char('p' | 'P') => self.events.send(AppEvent::ShowPlayer),
            KeyCode::Char('s' | 'S') => self.events.send(AppEvent::FocusShotgun),
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

    pub fn handle_mouse_events(&mut self, mouse_event: MouseEvent) -> color_eyre::Result<()> {
        match mouse_event.kind {
            MouseEventKind::ScrollUp => {
                self.send_log(Some("scrolling up".to_string()));
                self.events.send(AppEvent::ScrollUp)
            },
            MouseEventKind::Drag(mouse_button) => {
                match mouse_button {
                    MouseButton::Left => {
                        self.send_log(Some("left dragging".to_string()));
                    },
                    _ => {
                        self.send_log(Some("some other dragging".to_string()));
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
        self.send_log(log);
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
