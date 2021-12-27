use super::{Attachment, Embed, User};
use twilight_model::channel::Message as DiscordMessage;

#[derive(Clone)]
pub struct Message {
    pub attachments: Vec<Attachment>,
    pub author: User,
    pub channel_id: u64,
    pub content: String,
    pub embeds: Vec<Embed>,
}

impl Message {
    pub fn reply(&self, content: &str) -> Self {
        Self {
            content: content.to_string(),
            ..self.clone()
        }
    }

    pub fn set_embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = embeds;
        self
    }
}

impl From<DiscordMessage> for Message {
    fn from(message: DiscordMessage) -> Self {
        Message {
            attachments: message.attachments.into_iter().map(Into::into).collect(),
            author: message.author.into(),
            channel_id: message.channel_id.0.into(),
            content: message.content,
            embeds: message.embeds.into_iter().map(Into::into).collect(),
        }
    }
}

pub type IncomingMessage = (u64, Message);
