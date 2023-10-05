use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use crossterm::event::{self, Event};
use nom::bytes::complete::{tag, tag_no_case, take_till};
use nom::combinator::{map, opt};
use nom::sequence::separated_pair;
use nom::IResult;
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

    fn filter(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(tag_no_case("filter"), tag(":"), take_till(|c| c == ' '))(input)
    }

    fn parse_name(input: &str) -> IResult<&str, Option<(&str, &str)>> {
        map(opt(Self::filter), |filter| filter.map(|a| (a.0, a.1)))(input)
    }

    pub fn get_book(&self, name: impl Into<String>) {
        let sender = self.sender.clone();
        let base_url = self.base_url;
        let name = name.into();
        let client = self.client.clone();
        self.threadpool.execute(move || {
            if let Err(e) = Self::send_book(sender.clone(), client, base_url, name) {
                if let Err(e) = sender.send(Message::Key(e.to_string())) {
                    eprintln!("{:?}", e);
                }
            }
        })
    }

    fn send_book(
        sender: Sender<Message>,
        client: Client,
        base_url: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<()> {
        sender.send(Message::Key("loading...".into()))?;

        let input = name.into();
        let (name, filter) = Self::parse_name(&input).map_err(|_| anyhow!("parse name failed"))?;

        let base_url = base_url.into();
        let name = name.trim();
        let url = format!("{}/operate/search/{}", &base_url, name);

        let html = client
            .get(url)
            .send()
            .map_err(|_| anyhow!("send request failed"))?
            .text()
            .map_err(|_| anyhow!("get response text failed"))?;
        let doc = Html::parse_document(&html);
        let mut books = Vec::new();
        let Ok(selector) = Selector::parse("li.list-group-item h5 a") else {
            return Err(anyhow!("parse selector failed"));
        };
        for book in doc.select(&selector) {
            let Some(url) = book.value().attr("href") else {
                continue;
            };
            let url = format!("{}{}", base_url, url);
            let name = book.inner_html();

            books.push(Book::new(name, url));
        }

        if let Some((key, filter)) = filter {
            if key == "filter" {
                books.retain(|book| book.name().to_lowercase().contains(filter));
            }
        }

        sender
            .send(Message::Book((name.to_string(), books)))
            .map_err(|_| anyhow!("send message failed"))?;

        Ok(())
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        self.threadpool.join();
    }
}
