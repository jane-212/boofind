use std::sync::mpsc::{channel, Receiver};

use anyhow::{Context, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use tui_textarea::{Input, Key};

use crate::backend::{Backend, KegelState, Message};
use crate::layout::{books, footer, title};
use crate::term::Term;
pub use state::{Book, Mode, State};

mod state;

pub struct App<'a> {
    term: Term,
    should_quit: bool,
    backend: Backend,
    task: Receiver<Message>,
    state: State<'a>,
    event: Receiver<Event>,
}

impl<'a> App<'a> {
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
            self.event()?;
            self.update();
            self.draw()?;
        }
        self.term.restore().context("restore terminal failed")?;

        Ok(())
    }

    fn update(&mut self) {
        if let Ok(message) = self.task.try_recv() {
            match message {
                Message::Book(books) => {
                    self.state.books_mut().clear();
                    self.state.books_mut().extend(books);
                    self.state.reset();
                }
                Message::More(books) => {
                    self.state.books_mut().extend(books);
                }
                Message::Key(key) => {
                    self.state.set_key(&key);
                }
                Message::Kegel(kegel) => {
                    self.state.set_kegel(kegel);
                }
            }
        }
    }

    fn event(&mut self) -> Result<()> {
        if let Ok(key) = self.event.try_recv() {
            match self.state.mode() {
                Mode::Normal => match key {
                    Event::Key(KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: KeyModifiers::NONE,
                        ..
                    }) => *self.state.mode_mut() = Mode::Search,
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    }) => self.should_quit = true,
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('g'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    }) => {
                        if KegelState::End == *self.state.kegel() {
                            self.backend.kegel();
                        }
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('j'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    }) => self.state.select_next(1),
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('J'),
                        modifiers: KeyModifiers::SHIFT,
                        ..
                    }) => self.state.select_next(5),
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('k'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    }) => self.state.select_prev(1),
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('K'),
                        modifiers: KeyModifiers::SHIFT,
                        ..
                    }) => self.state.select_prev(5),
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('o'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    }) => {
                        if let Some(selected) = self.state.selected_book().selected() {
                            let url = self.state.books()[selected].url();
                            open::that(url)
                                .with_context(|| format!("open url [{}] failed", url))?;
                        }
                    }
                    _ => (),
                },
                Mode::Search => match key.into() {
                    Input {
                        key: Key::Enter,
                        ctrl: false,
                        alt: false,
                    } => {
                        if self.state.key() != "loading..." {
                            let search = self.state.reset_search();
                            self.backend.get_book(search);
                        }
                    }
                    Input {
                        key: Key::Esc,
                        ctrl: false,
                        alt: false,
                    } => *self.state.mode_mut() = Mode::Normal,
                    input => {
                        self.state.input(input);
                    }
                },
            }
        }

        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.term.terminal_mut().draw(|frame| {
            match self.state.mode() {
                Mode::Normal => (),
                Mode::Search => {
                    if self.state.search().is_empty() {
                        frame.set_cursor(2, 1);
                    }
                }
            };
            let area = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(3),
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .split(frame.size());

            let input = self.state.search().widget();
            frame.render_widget(input, area[0]);

            let title = title::Title::new(&self.state);
            frame.render_widget(title, area[1]);

            let hint = match self.state.kegel() {
                KegelState::Process(_, _, _) => Some(Color::Red),
                KegelState::Relax(_, _, _) => Some(Color::Green),
                KegelState::End => None,
            };
            let mut books = books::Books::new(&self.state).widget();
            if let Some(color) = hint {
                books = books.style(Style::new().fg(color).bg(color));
            }
            frame.render_stateful_widget(books, area[2], &mut self.state.selected_book());

            let footer = footer::Footer::new(&self.state);
            frame.render_widget(footer, area[3]);
        })?;

        Ok(())
    }
}
