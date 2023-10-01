use std::sync::mpsc::{channel, Receiver};

use anyhow::{Context, Result};
use crossterm::event::{Event, KeyCode};

use crate::backend::{Backend, Message};
use crate::layout::Root;
use crate::term::Term;
pub use state::{Book, Mode, State};

mod state;

pub struct App {
    term: Term,
    should_quit: bool,
    backend: Backend,
    task: Receiver<Message>,
    state: State,
    event: Receiver<Event>,
}

impl App {
    pub fn new() -> Result<Self> {
        let term = Term::new().context("new Term failed")?;
        let (task_sender, task_receiver) = channel();
        let (event_sender, event_receiver) = channel();
        let backend = Backend::new(task_sender, event_sender).context("init backend failed")?;
        let state = State::new();

        Ok(Self {
            term,
            should_quit: false,
            backend,
            task: task_receiver,
            state,
            event: event_receiver,
        })
    }

    pub fn run(mut self) -> Result<()> {
        while !self.should_quit {
            self.event();
            self.update();
            self.draw()?;
        }
        self.backend.join();
        self.term.restore().context("restore terminal failed")?;

        Ok(())
    }

    fn update(&mut self) {
        if let Ok(message) = self.task.try_recv() {
            match message {
                Message::Book(books) => {
                    self.state.books_mut().clear();
                    self.state.books_mut().extend(books);
                }
            }
        }
    }

    fn event(&mut self) {
        if let Ok(Event::Key(key)) = self.event.try_recv() {
            match self.state.mode() {
                Mode::Normal => match key.code {
                    KeyCode::Enter => *self.state.mode_mut() = Mode::Search,
                    KeyCode::Char('q') => self.should_quit = true,
                    _ => (),
                },
                Mode::Search => match key.code {
                    KeyCode::Enter => {
                        let search = self.state.search().to_string();
                        self.state.key_mut().clear();
                        self.state.key_mut().push_str(&search);
                        self.backend.get_book(search);
                        self.state.search_mut().clear();
                        *self.state.search_cursor_mut() = 2;
                    }
                    KeyCode::Esc => *self.state.mode_mut() = Mode::Normal,
                    KeyCode::Backspace => {
                        if !self.state.search().is_empty() {
                            let pos = *self.state.search_cursor();
                            if pos > 2 {
                                *self.state.search_cursor_mut() = pos.saturating_sub(1);
                            }
                            self.state.search_mut().remove(pos - 3);
                        }
                    }
                    KeyCode::Left => {
                        let pos = *self.state.search_cursor();
                        if pos > 2 {
                            *self.state.search_cursor_mut() = pos.saturating_sub(1);
                        }
                    }
                    KeyCode::Right => {
                        let pos = *self.state.search_cursor();
                        if pos < self.state.search().len() + 2 {
                            *self.state.search_cursor_mut() = pos.saturating_add(1);
                        }
                    }
                    KeyCode::Char(c) => {
                        self.state.search_mut().push(c);
                        let pos = self.state.search_cursor();
                        *self.state.search_cursor_mut() = pos.saturating_add(1);
                    }
                    _ => {}
                },
            }
        }
    }

    fn draw(&mut self) -> Result<()> {
        self.term.terminal_mut().draw(|frame| {
            frame.render_widget(Root::new(&self.state), frame.size());
            match self.state.mode() {
                Mode::Normal => (),
                Mode::Search => frame.set_cursor(*self.state.search_cursor() as u16, 1),
            }
        })?;

        Ok(())
    }
}
