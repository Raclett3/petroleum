use crate::bot::{CommandHandler, Context, IncomingMessage, MessageHandler};
use async_trait::async_trait;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

struct LimitedQueue<T> {
    head: usize,
    queue: Vec<Option<T>>,
}

impl<T> LimitedQueue<T> {
    fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut queue = Vec::new();
        queue.resize_with(size, || None);

        Self { head: 0, queue }
    }

    fn push(&mut self, item: T) -> Option<T> {
        let popped = std::mem::replace(&mut self.queue[self.head], Some(item));
        self.head = (self.head + 1) % self.queue.len();
        popped
    }
}

#[derive(Default)]
struct Config {
    queues: BTreeMap<u64, LimitedQueue<u64>>,
}

pub struct HistoryWindow {
    config: Arc<Mutex<Config>>,
}

impl HistoryWindow {
    fn new(config: Arc<Mutex<Config>>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl MessageHandler for HistoryWindow {
    async fn on_message(
        &mut self,
        (message_id, message): &IncomingMessage,
        context: &Context,
    ) -> Result<(), Box<dyn Error>> {
        let channel_id = message.channel_id;

        let target = self
            .config
            .lock()
            .unwrap()
            .queues
            .get_mut(&channel_id)
            .and_then(|queue| queue.push(*message_id));

        if let Some(target) = target {
            context.callbacks.delete_message(channel_id, target).await
        } else {
            Ok(())
        }
    }
}

pub struct HistoryWindowConfigurator {
    config: Arc<Mutex<Config>>,
}

impl HistoryWindowConfigurator {
    fn new(config: Arc<Mutex<Config>>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl CommandHandler for HistoryWindowConfigurator {
    fn accepts(&self, command_name: &str) -> bool {
        command_name == "meslimit"
    }

    async fn handler(
        &mut self,
        args: &[&str],
        (_, message): &IncomingMessage,
        context: &Context,
    ) -> Result<(), Box<dyn Error>> {
        let reply = {
            let mut config = self.config.lock().unwrap();
            match args {
                &["enable", size] => {
                    if let Ok(size) = size.parse::<usize>() {
                        config
                            .queues
                            .insert(message.channel_id, LimitedQueue::new(size));
                        "有効化しました。"
                    } else {
                        "不正なパラメータです。"
                    }
                }
                _ => "不正なコマンドです。",
            }
        };

        context.callbacks.send_message(message.reply(reply)).await
    }
}

pub fn history_window_pair() -> (HistoryWindow, HistoryWindowConfigurator) {
    let config = Arc::new(Mutex::new(Config::default()));

    let history_window = HistoryWindow::new(config.clone());
    let history_window_configurator = HistoryWindowConfigurator::new(config);
    (history_window, history_window_configurator)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_limited_queue() {
        use super::LimitedQueue;

        let mut queue = LimitedQueue::new(3);
        assert_eq!(queue.push(1), None);
        assert_eq!(queue.push(2), None);
        assert_eq!(queue.push(3), None);
        assert_eq!(queue.push(4), Some(1));
        assert_eq!(queue.push(5), Some(2));
        assert_eq!(queue.push(6), Some(3));
        assert_eq!(queue.push(7), Some(4));
    }
}
