use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget};

pub struct State {
    hello: String,
}

impl State {
    pub fn new() -> Self {
        Self { hello: "no".into() }
    }

    pub fn hello(&mut self, content: impl Into<String>) {
        self.hello = content.into();
    }
}

pub struct Root<'a> {
    state: &'a State,
}

impl<'a> Root<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title("search")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        Paragraph::new("press `enter` to search".dark_gray())
            .block(block)
            .render(area, buf);
    }
}

impl<'a> Widget for Root<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(3),
                Constraint::Max(area.height - 3),
            ])
            .split(area);
        self.render_header(area[0], buf);

        let block = Block::new()
            .title("books")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        Paragraph::new(self.state.hello.as_str())
            .block(block)
            .alignment(Alignment::Center)
            .render(area[1], buf);
    }
}
