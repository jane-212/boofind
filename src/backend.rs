use std::sync::mpsc::Sender;
use std::thread::spawn;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use crossterm::event::{self, Event};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_till};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use threadpool::ThreadPool;

use crate::app::Book;

pub enum Message {
    Book(Vec<Book>),
    More(Vec<Book>),
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
            spawn(move || loop {
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
            base_url: "https://www.iyd.wang",
            client,
        })
    }

    fn alt(input: &str) -> IResult<&str, (&str, &str)> {
        alt((
            separated_pair(tag_no_case("filter"), tag(":"), take_till(|c| c == ' ')),
            separated_pair(tag_no_case("tag"), tag(":"), take_till(|c| c == ' ')),
        ))(input)
    }

    fn filter(input: &str) -> IResult<&str, Vec<Option<(&str, &str)>>> {
        map(
            tuple((
                many0(tag(" ")),
                opt(Self::alt),
                many0(tag(" ")),
                opt(Self::alt),
            )),
            |filter| vec![filter.1, filter.3],
        )(input)
    }

    fn parse_name(input: &str) -> Result<(Vec<(&str, &str)>, &str)> {
        let (name, filters) = Self::filter(input).map_err(|_| anyhow!("parse filter failed"))?;
        let filters = filters.into_iter().flatten().collect();

        Ok((filters, name))
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

    fn parse_books(
        client: Client,
        url: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Vec<Book>> {
        let Ok(html) = client.get(url.into()).query(&[("s", name.into())]).send() else {
            return Ok(Vec::new());
        };

        let html = html
            .text()
            .map_err(|_| anyhow!("get response text failed"))?;
        let doc = Html::parse_document(&html);
        let mut books = Vec::new();
        let selector =
            Selector::parse("main#main article").map_err(|_| anyhow!("parse selector failed"))?;
        for article in doc.select(&selector) {
            let selector = Selector::parse("figure.thumbnail span.cat a")
                .map_err(|_| anyhow!("parse selector failed"))?;
            let Some(cat) = article.select(&selector).next() else {
                continue;
            };
            let tag = cat.inner_html();

            let selector = Selector::parse("header.entry-header h2.entry-title a")
                .map_err(|_| anyhow!("parse selector failed"))?;
            let Some(a) = article.select(&selector).next() else {
                continue;
            };
            let name = a.inner_html();
            let Some(url) = a.value().attr("href") else {
                continue;
            };

            books.push(Book::new(name, url, tag));
        }

        Ok(books)
    }

    fn filter_books(mut books: Vec<Book>, filters: Vec<(&str, &str)>) -> Result<Vec<Book>> {
        for filter in filters {
            match filter.0 {
                "filter" => books.retain(|book| book.name().to_lowercase().contains(filter.1)),
                "tag" => books.retain(|book| book.tag().to_lowercase().contains(filter.1)),
                _ => (),
            }
        }

        Ok(books)
    }

    fn send_book(
        sender: Sender<Message>,
        client: Client,
        base_url: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<()> {
        let total = 3;
        let input = name.into();
        let (filters, name) = Self::parse_name(&input).map_err(|_| anyhow!("parse name failed"))?;

        let base_url = base_url.into();
        let name = name.trim();

        for i in 0..total {
            sender.send(Message::Key(format!("[{}/{}]loading...", i + 1, total)))?;

            let books =
                Self::parse_books(client.clone(), format!("{}/page/{}", base_url, i), name)?;
            if books.is_empty() {
                break;
            }
            let books = Self::filter_books(books, filters.clone())?;

            let message = if i == 0 {
                Message::Book(books)
            } else {
                Message::More(books)
            };
            sender
                .send(message)
                .map_err(|_| anyhow!("send message failed"))?;
        }

        sender.send(Message::Key(name.into()))?;

        Ok(())
    }
}
