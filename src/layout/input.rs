use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget};

use crate::app::{Mode, State};

pub struct Input<'a> {
    state: &'a State,
}

impl<'a> Input<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }
}

impl<'a> Widget for Input<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::new()
            .title("search")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text = if self.state.search().is_empty() {
            match self.state.mode() {
                Mode::Normal => "press `enter` to search".dark_gray(),
                Mode::Search => "".into(),
            }
        } else {
            self.state.search().into()
        };
        Paragraph::new(text).block(block).render(area, buf);
    }
}
