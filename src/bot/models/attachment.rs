use twilight_model::channel::Attachment as DiscordAttachment;

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
