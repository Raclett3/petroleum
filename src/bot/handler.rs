use super::{IncomingMessage, Message};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

#[async_trait]
pub trait ResponseCallbacks: Send + Sync {
    async fn send_message(&self, _: Message) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn delete_message(&self, _: u64, _: u64) -> Result<(), Box<dyn Error>> {
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

#[async_trait]
pub trait CommandHandler: Send + Sync {
    fn accepts(&self, command_name: &str) -> bool;

    async fn handler(
        &mut self,
        args: &[&str],
        message: &IncomingMessage,
        context: &Context,
    ) -> Result<(), Box<dyn Error>>;
}

fn parse_command(command: &str) -> Option<(&str, Vec<&str>)> {
    let mut parts = command.split(' ').filter(|x| !x.is_empty());

    if let Some(command) = parts.next() {
        let args = parts.collect();
        Some((command, args))
    } else {
        None
    }
}

#[async_trait]
impl<T: CommandHandler> MessageHandler for T {
    async fn on_message(
        &mut self,
        incoming: &IncomingMessage,
        context: &Context,
    ) -> Result<(), Box<dyn Error>> {
        let (_, message) = incoming;

        if let Some((command, args)) = parse_command(&message.content) {
            if self.accepts(command) {
                return self.handler(&args, incoming, context).await;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_command() {
        use super::parse_command;
        assert_eq!(
            parse_command("command arg1 arg2"),
            Some(("command", vec!["arg1", "arg2"])),
        );
        assert_eq!(
            parse_command(" command  arg1  arg2 "),
            Some(("command", vec!["arg1", "arg2"])),
        );
        assert_eq!(parse_command("command"), Some(("command", vec![])),);
        assert_eq!(parse_command(" "), None);
    }
}
