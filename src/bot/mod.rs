pub mod handler;
pub mod models;

pub use handler::*;
pub use models::*;

use diesel::PgConnection;
use std::sync::{Arc, Mutex};
use twilight_gateway::Event;

pub struct Bot {
    message_handlers: Vec<Box<dyn MessageHandler>>,
    context: Context,
}

impl Bot {
    pub fn new<T: ResponseCallbacks + 'static>(callbacks: T, db_conn: PgConnection) -> Self {
        Bot {
            message_handlers: Vec::new(),
            context: Context {
                callbacks: Arc::new(callbacks),
                db_conn: Arc::new(Mutex::new(db_conn)),
            },
        }
    }

    pub fn on_message(&mut self, handler: impl MessageHandler + 'static) {
        self.message_handlers.push(Box::new(handler))
    }

    pub async fn handle(&mut self, event: Event) {
        match event {
            Event::MessageCreate(msg) => {
                let incoming = (msg.id.0.into(), msg.0.into());
                for handler in self.message_handlers.iter_mut() {
                    if let Err(error) = handler.on_message(&incoming, &self.context).await {
                        println!("[ERROR] {}", error);
                    }
                }
            }
            _ => (),
        }
    }
}
