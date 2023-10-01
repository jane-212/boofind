use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use crossterm::event::{self, Event};
use ratatui::widgets::{List, ListItem, ListState};
use reqwest::blocking::Client;
use threadpool::ThreadPool;

use crate::app::Book;

pub enum Message {
    Book(Vec<Book>),
}

pub struct Backend {
    threadpool: ThreadPool,
    sender: Sender<Message>,
    client: Client,
    base_url: &'static str,
}

impl Backend {
    const MAX_WORKER: usize = 10;
    pub fn new(task_sender: Sender<Message>, event_sender: Sender<Event>) -> Result<Self> {
        let threadpool = ThreadPool::new(Self::MAX_WORKER);
        {
            thread::spawn(move || loop {
                if let Ok(has_event) = event::poll(Duration::from_millis(250)) {
                    if has_event {
                        if let Ok(event) = event::read() {
                            event_sender.send(event).unwrap();
                        }
                    }
                }
            });
        }
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .context("build backend client failed")?;

        Ok(Self {
            threadpool,
            sender: task_sender,
            base_url: "",
            client,
        })
    }

    pub fn join(self) {
        self.threadpool.join();
    }

    pub fn get_book(&self, _: impl Into<String>) {
        let sender = self.sender.clone();
        self.threadpool.execute(move || {
            thread::sleep(Duration::from_secs(2));
            sender
                .send(Message::Book(vec![
                    Book::new("hello1", "url"),
                    Book::new("hello2", "url"),
                    Book::new("hello1", "url"),
                    Book::new("hello2", "url"),
                    Book::new("hello1", "url"),
                    Book::new("hello2", "url"),
                ]))
                .unwrap();
        })
    }
}
