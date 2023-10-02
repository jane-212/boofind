use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span};
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
            Mode::Normal => " Normal ",
            Mode::Search => " Search ",
        };
        let key = self.state.key();
        let key = if key.is_empty() {
            " <empty> ".into()
        } else {
            format!(" {} ", key)
        };
        let width = (area.width as usize)
            .saturating_sub(
                mode.chars()
                    .map(|char| if char.is_ascii() { 1 } else { 2 })
                    .sum(),
            )
            .saturating_sub(
                key.chars()
                    .map(|char| if char.is_ascii() { 1 } else { 2 })
                    .sum(),
            )
            .saturating_sub(2);
        let text = vec![
            mode.fg(Color::Black).bg(Color::White),
            Span::from(" ".repeat(width)),
            key.fg(Color::Black).bg(Color::White),
        ];
        let block = Block::new()
            .padding(Padding::horizontal(1))
            .borders(Borders::NONE);

        Paragraph::new(Line::from(text))
            .block(block)
            .render(area, buf);
    }
}
