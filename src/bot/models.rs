use twilight_model::channel::Message as DiscordMessage;

#[derive(Clone)]
pub struct Message {
    pub channel_id: u64,
    pub content: String,
}

impl Message {
    pub fn reply(&self, content: &str) -> Self {
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

pub type IncomingMessage = (u64, Message);
