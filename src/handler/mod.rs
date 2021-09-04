pub mod ping;

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

pub trait MessageHandler {
    fn on_message(&self, message: &Message) -> Option<Message>;
}

impl<T: Fn(&Message) -> Option<Message>> MessageHandler for T {
    fn on_message(&self, message: &Message) -> Option<Message> {
        self(message)
    }
}

#[derive(Default)]
pub struct Bot<'a> {
    message_handlers: Vec<Box<dyn MessageHandler + 'a>>,
}

impl<'a> Bot<'a> {
    pub fn new() -> Self {
        Bot::default()
    }

    pub fn on_message(&mut self, handler: impl MessageHandler + 'a) {
        self.message_handlers.push(Box::new(handler))
    }

    pub fn handle(&self, event: Event) -> Vec<Message> {
        match event {
            Event::MessageCreate(msg) => {
                let msg = msg.0;
                self.message_handlers
                    .iter()
                    .flat_map(|handler| handler.on_message(&msg.clone().into()))
                    .collect()
            }
            _ => vec![],
        }
    }
}
