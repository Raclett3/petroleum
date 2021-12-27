use twilight_model::channel::Channel as DiscordChannel;

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
