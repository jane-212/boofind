use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, ListState, Padding};
use tui_textarea::TextArea;
use tui_textarea::{CursorMove, Input};

pub enum Mode {
    Normal,
    Search,
}

pub struct Book {
    name: String,
    url: String,
}

impl Book {
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

pub struct State<'a> {
    search: TextArea<'a>,
    books: Vec<Book>,
    selected_book: ListState,
    mode: Mode,
    key: String,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        let block = Block::new()
            .title("search")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let mut search = TextArea::default();
        search.set_block(block);
        search.set_placeholder_text("press <enter> to search");
        search.set_cursor_line_style(Style::new());

        Self {
            search,
            books: Vec::new(),
            selected_book: ListState::default(),
            mode: Mode::Normal,
            key: "".into(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn set_key(&mut self, key: impl Into<String>) {
        self.key = key.into();
    }

    pub fn selected_book(&self) -> ListState {
        self.selected_book.clone()
    }

    pub fn reset(&mut self) {
        self.selected_book.select(None);
    }

    pub fn select_prev(&mut self) {
        let i = match self.selected_book.selected() {
            Some(i) => {
                if i == 0 {
                    self.books.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected_book.select(Some(i));
    }

    pub fn select_next(&mut self) {
        let i = match self.selected_book.selected() {
            Some(i) => {
                if i >= self.books.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_book.select(Some(i));
    }

    pub fn reset_search(&mut self) -> String {
        let search = self.search.lines()[0].clone();
        self.search.move_cursor(CursorMove::End);
        self.search.delete_line_by_head();

        search
    }

    pub fn input(&mut self, input: Input) {
        self.search.input(input);
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn mode_mut(&mut self) -> &mut Mode {
        &mut self.mode
    }

    pub fn search(&self) -> &TextArea {
        &self.search
    }

    pub fn books(&self) -> &Vec<Book> {
        &self.books
    }

    pub fn books_mut(&mut self) -> &mut Vec<Book> {
        &mut self.books
    }
}
