use crate::bot::{IncomingMessage, Message};

pub fn ping((_, message): &IncomingMessage) -> Option<Message> {
    if message.content == "ping?" {
        Some(message.reply("pong!"))
    } else {
        None
    }
}
