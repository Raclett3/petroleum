use twilight_model::channel::embed::{Embed as DiscordEmbed, EmbedAuthor, EmbedFooter, EmbedImage};

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

impl From<Embed> for DiscordEmbed {
    fn from(embed: Embed) -> Self {
        let Embed {
            author_name,
            avatar_url,
            description,
            footer,
            image,
            ..
        } = embed;
        DiscordEmbed {
            author: author_name.map(|author_name| EmbedAuthor {
                icon_url: avatar_url,
                name: author_name,
                proxy_icon_url: None,
                url: None,
            }),
            color: None,
            description: description,
            fields: Vec::new(),
            footer: footer.map(|footer| EmbedFooter {
                icon_url: None,
                proxy_icon_url: None,
                text: footer,
            }),
            image: image.map(|image| EmbedImage {
                url: image,
                proxy_url: None,
                height: None,
                width: None,
            }),
            kind: "".to_string(),
            provider: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
            video: None,
        }
    }
}
