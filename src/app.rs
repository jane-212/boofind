use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode};

use crate::backend::{Backend, Message};
use crate::layout::{Root, State};
use crate::term::Term;

pub struct App {
    term: Term,
    should_quit: bool,
    backend: Backend,
    receiver: Receiver<Message>,
    state: State,
}

impl App {
    pub fn new() -> Result<Self> {
        let term = Term::new().context("new Term failed")?;
        let (sender, receiver) = channel();
        let backend = Backend::new(sender);
        let state = State::new();

        Ok(Self {
            term,
            should_quit: false,
            backend,
            receiver,
            state,
        })
    }

    pub fn run(mut self) -> Result<()> {
        loop {
            self.event()?;
            self.update();
            self.draw()?;

            if self.should_quit {
                break;
            }
        }
        self.backend.join();
        self.term.restore().context("restore terminal failed")?;

        Ok(())
    }
    
    fn update(&mut self) {
        if let Ok(message) = self.receiver.try_recv() {
            match message {
                Message::Hello(content) => self.state.hello(content),
            }
        }
    }

    fn draw(&mut self) -> Result<()> {
        self.term
            .terminal_mut()
            .draw(|frame| frame.render_widget(Root::new(&self.state), frame.size()))?;

        Ok(())
    }

    fn event(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                if KeyCode::Char('q') == key.code {
                    self.should_quit = true;
                }
                if KeyCode::Char('n') == key.code {
                    self.state.hello("loading...");
                    self.backend.get_page();
                }
            }
        }

        Ok(())
    }
}
