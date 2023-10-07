use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Padding};

use crate::app::State;

pub struct Books<'a> {
    state: &'a State<'a>,
}

impl<'a> Books<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }

    pub fn widget(self) -> List<'a> {
        let block = Block::new()
            .title("books")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .border_type(BorderType::Rounded);

        let items = self
            .state
            .books()
            .iter()
            .map(|book| {
                ListItem::new(Line::from(vec![
                    Span::from(format!("[{}]", book.tag())),
                    Span::from(book.name()),
                ]))
            })
            .collect::<Vec<ListItem>>();

        List::new(items)
            .highlight_style(Style::new().fg(Color::Black).bg(Color::White))
            .block(block)
    }
}
