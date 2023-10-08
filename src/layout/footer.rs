use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget};

use crate::app::{Mode, State};
use crate::backend::KegelState;

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
        let (key, key_bg) = if key.is_empty() {
            (" <empty> ".into(), Color::White)
        } else {
            (format!(" {} ", key), Color::Blue)
        };

        let (kegel, kegel_bg) = match self.state.kegel() {
            KegelState::Process(t, i, g) => (
                format!(" p[{}/{}] i[{}/{}] g[{}/{}] ", t.0, t.1, i.0, i.1, g.0, g.1),
                Color::Red,
            ),
            KegelState::Relax(t, i, g) => (
                format!(" r[{}/{}] i[{}/{}] g[{}/{}] ", t.0, t.1, i.0, i.1, g.0, g.1),
                Color::Green,
            ),
            KegelState::End => (" kegel ".into(), Color::White),
        };

        let text = vec![
            mode.fg(Color::Black),
            key.fg(Color::Black).bg(key_bg),
            kegel.fg(Color::Black).bg(kegel_bg),
        ];
        let block = Block::new()
            .padding(Padding::horizontal(1))
            .borders(Borders::NONE);

        Paragraph::new(Line::from(text))
            .block(block)
            .render(area, buf);
    }
}
