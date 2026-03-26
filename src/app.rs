//std library
use std::{collections::VecDeque, fmt::format};

use std::sync::Arc;
use tokio::sync::Mutex;

//ratatui
use ratatui::{
    DefaultTerminal,
    /* style::{ Color, Style, Stylize },
    widgets::{Block, Borders, Clear, Paragraph, Wrap, BorderType}, */
    Frame,
    crossterm::event::{
        self, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    },
    layout::{Constraint, Direction, Layout, Rect},
};

//crossterm
use crossterm::event::EnableMouseCapture;
use crossterm::{
    style::Color,
    terminal::{disable_raw_mode, enable_raw_mode},
};

//user made ones
use crate::components::enums::ReloadAmount;
use crate::components::enums::{ConfirmationType, GameState, ShotgunCycleView};
use crate::components::match_data::*;
use crate::components::shotgun::ShotgunCycle;
use crate::event::AppEvent::HideFocusedPopup;
use crate::event::{AppEvent, Event, EventHandler};
use crate::ui;
use crate::ui_components::logger::Logger;
use crate::ui_components::widget_data::{WidgetData, WidgetKind};
use crate::{data::Data, ui_components::widget_data};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// game data
    pub data: Data,
    /// screen state, are you in the game, in the menu, in a confirmation screen
    pub state: GameState,
    pub prev_state: Option<GameState>,
    /// match data
    pub match_data: MatchData,
    ///holds the information of the widgets
    pub widget_data: Arc<Mutex<WidgetData>>,
    /// logger will replace log, and it will automatically size to the correct screen size
    pub logger: Logger,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            data: Data::new(),
            state: GameState::default(),
            prev_state: None,
            match_data: MatchData::new(),
            widget_data: Arc::new(Mutex::new(WidgetData::new())),
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
            if self.logger.len() >= max_size {
                self.logger.pop_front();
            }
            self.logger.send_log(Some(msg));
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

        while self.running {
            let widget_data_snapshot = {
                let widget_data = self.widget_data.lock().await;
                widget_data.clone()
            };

            terminal.draw(|frame| {
                //changing logger's capacity
                let area = frame.area();
                let max_window_lines = (area.height as f32 / 1.45) as usize;
                self.logger.set_window_size(max_window_lines);
                self.logger.update_window();
                self.app_render_ui(frame, &widget_data_snapshot)
            })?;

            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    crossterm::event::Event::Mouse(mouse_event) => {
                        self.handle_mouse_events(mouse_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),

                    AppEvent::Reload(amount) => {
                        if let Some(msg) = self
                            .data
                            .shotgun
                            .reload_random_shells(amount.as_usize())
                            .await
                        {
                            self.logger.send_log(Some(msg));

                            let cycle = self.data.shotgun.cycle.lock().await;
                            let mut widget_data = self.widget_data.lock().await;
                            widget_data.shotgun_cycle_view = match *cycle {
                                ShotgunCycle::Ready => ShotgunCycleView::Ready,
                                ShotgunCycle::Shooting => ShotgunCycleView::Shooting,
                                ShotgunCycle::Reloading => ShotgunCycleView::Reloading,
                                ShotgunCycle::Blanking => ShotgunCycleView::Blanking,
                                _ => ShotgunCycleView::Ready,
                            };
                        }
                    }

                    AppEvent::Shoot => {
                        if let Some(msg) =
                            self.data.shotgun.shoot(Arc::clone(&self.widget_data)).await
                        {
                            //not a very robust solution but it works for now
                            if msg.contains("Last") {
                                self.match_data.next_round();
                            }
                            self.logger.send_log(Some(msg));
                        }
                    }

                    AppEvent::ShowPopup(kind) => {
                        let kind = match kind {
                            Some(_) => kind.unwrap(),
                            _ => continue,
                        };

                        let mut widget_data = self.widget_data.lock().await;
                        widget_data.display_widget(kind, true);
                        widget_data.render_stack.push(kind);
                    }

                    //kind is Option<WidgetKind>
                    AppEvent::HideFocusedPopup => {
                        //widget logic
                        // Lock widget_data once
                        let widget_data = self.widget_data.lock().await;

                        // Get the focused widget and copy it out
                        let kind_opt = widget_data.get_focus();
                        match kind_opt {
                            Some(WidgetKind::Shotgun) => {
                                drop(widget_data);
                            }
                            Some(WidgetKind::Player(_)) => {
                                drop(widget_data);
                            }
                            _ => {
                                if let Some(kind) = kind_opt {
                                    // Drop the lock before calling send_log
                                    drop(widget_data);

                                    // Now it's safe to call send_log
                                    self.send_log(Some(format!("hiding {:?}", kind)));

                                    // Re-lock to modify widget_data
                                    let mut widget_data = self.widget_data.lock().await;

                                    // Hide the widget if it's displayed
                                    if widget_data.is_displayed(kind) {
                                        widget_data.hide_widget(kind);
                                    }

                                    // Update render stack
                                    widget_data.render_stack.retain(|k| *k != kind);

                                    // Focus next if anything is left
                                    if !widget_data.render_stack.is_empty() {
                                        widget_data.focus_next();
                                    }
                                }
                            }
                        }
                    }

                    AppEvent::Confirmation(Some(confirmation_type)) => {
                        let mut widget_data = self.widget_data.lock().await;

                        match confirmation_type {
                            ConfirmationType::Quit => {
                                widget_data.display_widget_with_content(
                                    WidgetKind::Confirmation,
                                    String::from("Press 'q' to confirm"),
                                );
                                self.prev_state = Some(self.state.clone());
                                self.state = GameState::Confirmation(ConfirmationType::Quit);
                            }
                            _ => {}
                        }
                    }

                    AppEvent::FocusShotgun => {
                        let mut widget_data = self.widget_data.lock().await;
                        widget_data.toggle_focus(WidgetKind::Shotgun);
                    }

                    AppEvent::ChangePlayerTurn(num) => {
                        let mut widget_data = self.widget_data.lock().await;
                        widget_data.kind_focus(WidgetKind::Player(num));
                        self.match_data.update_turn(Some(num));
                        self.logger.send_log(Some(format!(
                            "now {}'s turn {:?}",
                            num, self.match_data.player_turn
                        )));
                    }

                    //make this into a scroll wheel as well
                    AppEvent::ScrollUp => {
                        /* if self.logger.log_scroll > 0 {
                            self.logger.scroll_up();
                        } */
                    }
                    AppEvent::ScrollDown => {
                        /*                         self.logger.scroll_down(); */
                    }

                    AppEvent::ChangeFocus => {
                        let mut widget_data = self.widget_data.lock().await;
                        widget_data.focus_next();
                    }
                    AppEvent::ChangeFocusBack => {
                        let mut widget_data = self.widget_data.lock().await;
                        widget_data.focus_prev();
                    }
                    _ => {
                        self.logger
                            .send_log(Some(String::from("Failure to catch event")));
                    }
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match &self.state {
            GameState::Menu => self.handle_menu_events(key_event),
            GameState::Game => self.handle_game_events(key_event),
            GameState::Settings => self.handle_settings_events(key_event),
            GameState::Confirmation(confirmation_type) => match confirmation_type {
                ConfirmationType::Quit => self.handle_quit_confirmation(key_event),
                _ => self.handle_default_confirmation(key_event),
            },
        }
        Ok(())
    }

    pub fn handle_menu_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('s' | 'S') => self.events.send(AppEvent::StartGame),
            KeyCode::Char('q' | 'Q') => self
                .events
                .send(AppEvent::Confirmation(Some(ConfirmationType::Quit))),
            _ => todo!(),
        }
    }

    pub fn handle_settings_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q' | 'Q') => self
                .events
                .send(AppEvent::Confirmation(Some(ConfirmationType::Quit))),
            _ => {}
        }
    }

    pub fn handle_default_confirmation(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q' | 'Q') => self.events.send(AppEvent::Quit),
            _ => {
                self.state = self.prev_state.take().unwrap();
                self.events.send(AppEvent::HideFocusedPopup);
            }
        }
    }

    pub fn handle_quit_confirmation(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q' | 'Q') => self.events.send(AppEvent::Quit),
            _ => {
                self.state = self.prev_state.take().unwrap();
                self.events.send(AppEvent::HideFocusedPopup);
            }
        }
    }

    pub fn handle_game_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q' | 'Q') => self
                .events
                .send(AppEvent::Confirmation(Some(ConfirmationType::Quit))),

            //ui keys
            KeyCode::Char('d' | 'D') => self
                .events
                .send(AppEvent::ShowPopup(Some(WidgetKind::Data))),

            KeyCode::Char('l' | 'L') => {
                self.events.send(AppEvent::ShowPopup(Some(WidgetKind::Log)));
            }

            //KeyCode::Char('i' | 'I') => self.events.send(AppEvent::ShowInventory),
            // KeyCode::Char('p' | 'P') => self.events.send(AppEvent::ShowPopup(Some(WidgetKind::Player))),
            // KeyCode::Char('s' | 'S') => self.events.send(AppEvent::FocusShotgun),
            KeyCode::Char('x' | 'X') => self.events.send(AppEvent::HideFocusedPopup),
            KeyCode::Char('k') => self.events.send(AppEvent::ScrollUp),
            KeyCode::Char('j') => self.events.send(AppEvent::ScrollDown),

            //need to find a key that isn't escaped
            // KeyCode::Char('t') => self.events.send(AppEvent::ChangeFocusBack),
            KeyCode::Tab => self.events.send(AppEvent::ChangeFocus),

            //game keys
            KeyCode::Char('r' | 'R') => match self.match_data.round_count() {
                1 => self.events.send(AppEvent::Reload(ReloadAmount::One)),
                2 => self.events.send(AppEvent::Reload(ReloadAmount::Two)),
                3 => self.events.send(AppEvent::Reload(ReloadAmount::Three)),
                4 => self.events.send(AppEvent::Reload(ReloadAmount::Four)),
                5 => self.events.send(AppEvent::Reload(ReloadAmount::Five)),
                _ => self.events.send(AppEvent::Reload(ReloadAmount::Five)),
            },
            KeyCode::Char(' ') => self.events.send(AppEvent::Shoot),
            KeyCode::Char('1') => {
                self.events.send(AppEvent::ChangePlayerTurn(0));
            }
            KeyCode::Char('2') => {
                self.events.send(AppEvent::ChangePlayerTurn(1));
            }
            // Other handlers you could add here.
            _ => {}
        }
    }

    pub fn handle_mouse_events(&mut self, mouse_event: MouseEvent) -> color_eyre::Result<()> {
        match mouse_event.kind {
            MouseEventKind::ScrollUp => {
                self.logger.send_log(Some("scrolling up".to_string()));
                self.events.send(AppEvent::ScrollUp)
            }
            MouseEventKind::Drag(mouse_button) => match mouse_button {
                MouseButton::Left => {
                    self.logger.send_log(Some("left dragging".to_string()));
                }
                _ => {
                    self.logger
                        .send_log(Some("some other dragging".to_string()));
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn app_render_ui(&mut self, frame: &mut Frame, widget_data: &WidgetData) {
        let log: Option<String> = ui::render_ui(self, frame, widget_data);
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
