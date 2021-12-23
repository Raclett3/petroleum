pub mod ping;

use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use twilight_gateway::Event;
use twilight_model::channel::Message as DiscordMessage;

#[derive(Clone)]
pub struct Message {
    pub channel_id: u64,
    pub content: String,
}

impl Message {
    fn reply(&self, content: &str) -> Self {
        Self {
            content: content.to_string(),
            ..self.clone()
        }
    }
}

impl From<DiscordMessage> for Message {
    fn from(message: DiscordMessage) -> Self {
        Message {
            channel_id: message.channel_id.0,
            content: message.content,
        }
    }
}

pub struct Context {
    callbacks: Arc<dyn ResponseCallbacks>,
}

#[async_trait]
pub trait MessageHandler {
    async fn on_message(&self, message: &Message, context: &Context) -> Result<(), Box<dyn Error>>;
}

pub struct FnMessageHandler<T: Fn(&Message) -> Option<Message> + Send + Sync>(pub T);

#[async_trait]
impl<T: Fn(&Message) -> Option<Message> + Send + Sync> MessageHandler for FnMessageHandler<T> {
    async fn on_message(&self, message: &Message, context: &Context) -> Result<(), Box<dyn Error>> {
        if let Some(reply) = self.0(message) {
            context.callbacks.send_message(reply).await
        } else {
            Ok(())
        }
    }
}

#[async_trait]
pub trait ResponseCallbacks: Send + Sync {
    async fn send_message(&self, _: Message) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct Bot<'a> {
    message_handlers: Vec<Box<dyn MessageHandler + 'a>>,
    context: Context,
}

impl<'a> Bot<'a> {
    pub fn new<T: ResponseCallbacks + 'static>(callbacks: T) -> Self {
        Bot {
            message_handlers: Vec::new(),
            context: Context {
                callbacks: Arc::new(callbacks),
            },
        }
    }

    pub fn on_message(&mut self, handler: impl MessageHandler + 'a) {
        self.message_handlers.push(Box::new(handler))
    }

    pub async fn handle(&self, event: Event) {
        match event {
            Event::MessageCreate(msg) => {
                let msg = msg.0.into();
                for handler in self.message_handlers.iter() {
                    if let Err(error) = handler.on_message(&msg, &self.context).await {
                        println!("[ERROR] {}", error);
                    }
                }
            }
            _ => (),
        }
    }
}
