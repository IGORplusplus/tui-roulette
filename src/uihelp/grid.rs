use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub struct Grid {
    pub cols: usize,
    pub rows: usize,
}

impl Widget for Grid {

    fn render(self, area: Rect, buf: &mut Buffer) {
    }
}
