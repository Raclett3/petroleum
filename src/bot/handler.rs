use super::{IncomingMessage, Message};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

#[async_trait]
pub trait ResponseCallbacks: Send + Sync {
    async fn send_message(&self, _: Message) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct Context {
    pub callbacks: Arc<dyn ResponseCallbacks>,
}

#[async_trait]
pub trait MessageHandler {
    async fn on_message(
        &mut self,
        message: &IncomingMessage,
        context: &Context,
    ) -> Result<(), Box<dyn Error>>;
}

pub struct FnMessageHandler<T: FnMut(&IncomingMessage) -> Option<Message> + Send + Sync>(pub T);

#[async_trait]
impl<T> MessageHandler for FnMessageHandler<T>
where
    T: FnMut(&IncomingMessage) -> Option<Message> + Send + Sync,
{
    async fn on_message(
        &mut self,
        message: &IncomingMessage,
        context: &Context,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(reply) = self.0(message) {
            context.callbacks.send_message(reply).await
        } else {
            Ok(())
        }
    }
}
