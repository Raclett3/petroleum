use crate::bot::Message;

pub fn ping(message: &Message) -> Option<Message> {
    if message.content == "ping?" {
        Some(message.reply("pong!"))
    } else {
        None
    }
}
