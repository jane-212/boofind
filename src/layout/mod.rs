use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Widget;

use crate::app::State;

mod books;
mod input;
mod footer;

pub struct Root<'a> {
    state: &'a State,
}

impl<'a> Root<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }
}

impl<'a> Widget for Root<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        input::Input::new(self.state).render(area[0], buf);
        books::Books::new(self.state).render(area[1], buf);
        footer::Footer::new(self.state).render(area[2], buf);
    }
}
