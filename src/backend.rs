use std::sync::mpsc::Sender;

use threadpool::ThreadPool;

pub enum Message {
    Hello(String),
}

pub struct Backend {
    threadpool: ThreadPool,
    sender: Sender<Message>,
}

impl Backend {
    const MAX_WORKER: usize = 10;
    pub fn new(sender: Sender<Message>) -> Self {
        let threadpool = ThreadPool::new(Self::MAX_WORKER);

        Self { threadpool, sender }
    }

    pub fn join(self) {
        self.threadpool.join();
    }

    pub fn get_page(&self) {
        let sender = self.sender.clone();
        self.threadpool.execute(move || {
            sender.send(Message::Hello("loading...".into())).unwrap();
            sender.send(Message::Hello("hello".into())).unwrap();
        })
    }
}
