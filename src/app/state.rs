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

pub struct State {
    search: String,
    search_cursor: usize,
    books: Vec<Book>,
    mode: Mode,
    key: String,
}

impl State {
    pub fn new() -> Self {
        Self {
            search: "".into(),
            search_cursor: 2,
            books: Vec::new(),
            mode: Mode::Normal,
            key: "".into(),
        }
    }
    
    pub fn key(&self) -> &str {
        &self.key
    }
    
    pub fn key_mut(&mut self) -> &mut String {
        &mut self.key
    }
    
    pub fn mode(&self) -> &Mode {
        &self.mode
    }
    
    pub fn mode_mut(&mut self) -> &mut Mode {
        &mut self.mode
    }

    pub fn search(&self) -> &str {
        &self.search
    }
    
    pub fn search_mut(&mut self) -> &mut String {
        &mut self.search
    }

    pub fn search_cursor(&self) -> &usize {
        &self.search_cursor
    }

    pub fn search_cursor_mut(&mut self) -> &mut usize {
        &mut self.search_cursor
    }

    pub fn books(&self) -> &Vec<Book> {
        &self.books
    }

    pub fn books_mut(&mut self) -> &mut Vec<Book> {
        &mut self.books
    }
}
