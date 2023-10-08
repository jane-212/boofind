use ratatui::layout::Alignment;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget};

use crate::app::State;

pub struct Title<'a> {
    state: &'a State<'a>,
}

impl<'a> Title<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }
}

impl<'a> Widget for Title<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::new()
            .padding(Padding::horizontal(1))
            .borders(Borders::NONE);

        let key = self.state.key();
        let (key, key_bg) = if key.is_empty() {
            (" <empty> ".into(), Color::White)
        } else {
            (format!(" {} ", key), Color::White)
        };

        let text = vec![key.fg(Color::Black).bg(key_bg)];

        Paragraph::new(Line::from(text).alignment(Alignment::Center))
            .block(block)
            .render(area, buf);
    }
}
