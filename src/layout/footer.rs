use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget};

use crate::app::{Mode, State};

pub struct Footer<'a> {
    state: &'a State<'a>,
}

impl<'a> Footer<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }
}

impl<'a> Widget for Footer<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mode = match self.state.mode() {
            Mode::Normal => " Normal ".bg(Color::Green),
            Mode::Search => " Search ".bg(Color::Yellow),
        };

        let key = self.state.key();
        let key = if key.is_empty() {
            " <empty> ".into()
        } else {
            format!(" {} ", key)
        };

        let text = vec![mode.fg(Color::Black), key.fg(Color::Black).bg(Color::White)];
        let block = Block::new()
            .padding(Padding::horizontal(1))
            .borders(Borders::NONE);

        Paragraph::new(Line::from(text))
            .block(block)
            .render(area, buf);
    }
}
