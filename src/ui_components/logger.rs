use std::collections::VecDeque;

use std::time::{Duration, Instant};

use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Layout, Margin},
    style::{Color, Style, Stylize},
    symbols::scrollbar,
    text::{Line, Masked, Span},
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

#[derive(Debug)]
pub struct Logger {
    pub log: VecDeque<String>,
    history_size: usize,
    //display specific
    window_size: usize,
    window: Vec<String>,
    pub log_scroll: usize,
}

#[derive(Default)]
struct ScrollState {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            log: VecDeque::new(),
            history_size: 250,
            window_size: 0,
            window: Vec::new(),
            log_scroll: 0,
        }
    }

    pub fn send_log(&mut self, message: Option<String>) {
        if let Some(msg) = message {
            if self.log.len() >= self.history_size {
                self.log.pop_front();
            }
            self.log.push_back(msg)
        }

        /* let scroll_size = self.window_size.saturating_sub(self.window.len());
        if self.window.len() >= self.window_size && self.log_scroll < scroll_size {
            self.scroll_down();
        } */

        /* if self.window.len() >= self.window_size {
            self.log_scroll = self.window.len().saturating_sub(self.window_size);
        }  */
        /* let max_scroll = self.window.len().saturating_sub(self.window_size);
        if self.log_scroll >= max_scroll.saturating_sub(1) {
            self.log_scroll = max_scroll;
        } */
        let max_scroll = self.window.len().saturating_sub(self.window_size);

        // Only scroll if log_scroll == current max_scroll before adding new log
        if self.log_scroll == max_scroll {
            self.log_scroll = self.window.len().saturating_sub(self.window_size);
        }
    }

    pub fn set_window_size(&mut self, line_number: usize) {
        self.window_size = line_number;
    }

    pub fn scroll_down(&mut self) {
        if self.log_scroll < self.window.len() {
            self.log_scroll += 1;
            self.update_window();
        }
    }

    pub fn scroll_up(&mut self) {
        if self.log_scroll > 0 {
            self.log_scroll -= 1;
            self.update_window();
        }
    }

    pub fn update_window(&mut self) {
        let total = self.log.len();

        self.log_scroll = self.log_scroll.min(total);

        let end = total.saturating_sub(self.log_scroll);
        let start = end.saturating_sub(self.window_size);

        self.window = self
            .log
            .iter()
            .skip(start)
            .take(end - start)
            .cloned()
            .collect();
    }

    pub fn get_window(&self) -> &[String] {
        &self.window
    }

    pub fn pop_front(&mut self) {
        self.log.pop_front();
    }

    pub fn len(&self) -> usize {
        self.log.len()
    }
}
