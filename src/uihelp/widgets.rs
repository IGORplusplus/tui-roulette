pub struct MyWidget {
    content: String,
}

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), &self.content, Style::default().fg(Color::Green));
    }
}
