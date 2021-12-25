use twilight_model::{
    channel::embed::Embed as DiscordEmbed, channel::Attachment as DiscordAttachment,
    channel::Channel as DiscordChannel, channel::Message as DiscordMessage,
    user::User as DiscordUser,
};

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

#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub avatar: Option<String>,
}

impl From<DiscordUser> for User {
    fn from(user: DiscordUser) -> Self {
        User {
            id: user.id.0.into(),
            name: user.name,
            avatar: user.avatar,
        }
    }
}

#[derive(Clone)]
pub struct Attachment {
    pub filename: String,
    pub url: String,
}

impl From<DiscordAttachment> for Attachment {
    fn from(attachment: DiscordAttachment) -> Self {
        Attachment {
            filename: attachment.filename,
            url: attachment.url,
        }
    }
}

#[derive(Clone)]
pub struct Channel {
    pub id: u64,
    pub name: String,
}

impl From<DiscordChannel> for Channel {
    fn from(channel: DiscordChannel) -> Self {
        Channel {
            id: channel.id().0.into(),
            name: channel.name().unwrap_or("").to_owned(),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Embed {
    pub author_name: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub footer: Option<String>,
    pub image: Option<String>,
}

impl Embed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn author_name(mut self, author_name: String) -> Self {
        self.author_name = Some(author_name);
        self
    }

    pub fn avatar_url(mut self, avatar_url: String) -> Self {
        self.avatar_url = Some(avatar_url);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn footer(mut self, footer: String) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn image(mut self, image: String) -> Self {
        self.image = Some(image);
        self
    }
}

impl From<DiscordEmbed> for Embed {
    fn from(embed: DiscordEmbed) -> Self {
        let (author_name, avatar_url) = embed
            .author
            .map_or((None, None), |author| (Some(author.name), author.icon_url));
        Embed {
            author_name,
            avatar_url,
            description: embed.description,
            footer: embed.footer.map(|x| x.text),
            image: embed.image.map(|x| x.url),
        }
    }
}
