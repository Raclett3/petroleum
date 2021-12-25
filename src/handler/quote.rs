use crate::bot::{Context, Embed, IncomingMessage, MessageHandler};
use async_trait::async_trait;
use chrono::Local;
use once_cell::sync::OnceCell;
use regex::{Match, Regex};
use std::error::Error;

async fn message_to_quote(
    context: &Context,
    channel_id: Option<u64>,
    message_id: Option<u64>,
) -> Option<(Embed, Vec<Embed>)> {
    let channel = context.callbacks.fetch_channel(channel_id?).await?;
    let message = context
        .callbacks
        .fetch_message(channel_id?, message_id?)
        .await?;

    let author = message.author;
    let author_id = author.id;
    let avatar_url = author
        .avatar
        .map(|avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                author_id, avatar
            )
        })
        .unwrap_or_else(|| "https://cdn.discordapp.com/embed/avatars/0.png".to_string());
    let now = Local::now().format("%Y/%m/%d %H:%M:%D").to_string();

    let mut embed = Embed::new()
        .author_name(author.name)
        .avatar_url(avatar_url)
        .description(message.content)
        .footer(format!("#{} - {}", channel.name, now));

    if let Some(attachment) = message.attachments.into_iter().next() {
        if !attachment.filename.starts_with("SPOILER_") {
            embed = embed.image(attachment.url);
        }
    }

    Some((embed, message.embeds))
}

pub struct Quote;

#[async_trait]
impl MessageHandler for Quote {
    async fn on_message(
        &mut self,
        (_, message): &IncomingMessage,
        context: &Context,
    ) -> Result<(), Box<dyn Error>> {
        static REGEX: OnceCell<Regex> = OnceCell::new();

        let regex = REGEX.get_or_init(|| {
            Regex::new(
                r#"discord(app)?\.com/channels/[0-9]+/(?P<channelId>[0-9]+)/(?P<messageId>[0-9]+)"#,
            )
            .unwrap()
        });

        let mut quotes = Vec::new();
        let mut quoted_embeds = Vec::new();

        for capture in regex.captures_iter(&message.content) {
            let parse = |x: Option<Match<'_>>| x.and_then(|x| x.as_str().parse().ok());
            let channel_id = parse(capture.name("channelId"));
            let message_id = parse(capture.name("messageId"));

            if let Some((quote, mut embeds)) =
                message_to_quote(&context, channel_id, message_id).await
            {
                quotes.push(quote);
                quoted_embeds.append(&mut embeds);
            }
        }

        if quotes.is_empty() {
            return Ok(());
        }

        let reply = message.reply("").set_embeds(quotes);
        context.callbacks.send_message(reply).await?;

        if !quoted_embeds.is_empty() {
            let reply = message.reply("embeds:").set_embeds(quoted_embeds);
            context.callbacks.send_message(reply).await?;
        }

        Ok(())
    }
}
