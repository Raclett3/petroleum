pub mod handler;
pub mod models;

pub use handler::*;
pub use models::*;

use std::sync::Arc;
use twilight_gateway::Event;

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
