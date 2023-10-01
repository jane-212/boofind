use ratatui::layout::Alignment;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};

use crate::app::State;

pub struct Books<'a> {
    state: &'a State,
}

impl<'a> Books<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }
}

impl<'a> Widget for Books<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::new()
            .title("books")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let text = self
            .state
            .books()
            .iter()
            .map(|book| Line::from(format!("name: {}", book.name())))
            .collect::<Vec<Line>>();

        Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
