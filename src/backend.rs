use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use crossterm::event::{self, Event};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use threadpool::ThreadPool;

use crate::app::Book;

pub enum Message {
    Book((String, Vec<Book>)),
    Key(String),
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
            let task_sender = task_sender.clone();
            thread::spawn(move || loop {
                if let Ok(has_event) = event::poll(Duration::from_millis(250)) {
                    if has_event {
                        if let Ok(event) = event::read() {
                            if let Err(e) = event_sender.send(event) {
                                if let Err(e) = task_sender.send(Message::Key(e.to_string())) {
                                    eprintln!("{:?}", e);
                                }
                            }
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
            base_url: "https://www.soushu.vip",
            client,
        })
    }

    pub fn get_book(&self, name: impl Into<String>) {
        let sender = self.sender.clone();
        let base_url = self.base_url;
        let name = name.into();
        let url = format!("{}/operate/search/{}", base_url, name);
        let client = self.client.clone();
        self.threadpool.execute(move || {
            if let Err(e) = Self::send_book(sender.clone(), url, client, base_url, name) {
                if let Err(e) = sender.send(Message::Key(e.to_string())) {
                    eprintln!("{:?}", e);
                }
            }
        })
    }

    fn send_book(
        sender: Sender<Message>,
        url: impl Into<String>,
        client: Client,
        base_url: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<()> {
        sender.send(Message::Key("loading...".into()))?;
        let html = client
            .get(url.into())
            .send()
            .context("send request failed")?
            .text()
            .context("get response text failed")?;
        let doc = Html::parse_document(&html);
        let mut books = Vec::new();
        let base_url = base_url.into();
        let Ok(selector) = Selector::parse("li.list-group-item h5 a") else {
            return Err(anyhow!("parse selector failed"));
        };
        for book in doc.select(&selector) {
            let Some(url) = book.value().attr("href") else {
                continue;
            };
            let url = format!("{}{}", base_url, url);
            let name = book.inner_html();

            books.push(Book::new(name.trim(), url));
        }

        sender
            .send(Message::Book((name.into(), books)))
            .context("send message failed")?;

        Ok(())
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        self.threadpool.join();
    }
}
