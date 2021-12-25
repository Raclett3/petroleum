mod bot;
mod handler;

use async_trait::async_trait;
use bot::Bot;
use bot::Channel;
use bot::Embed;
use bot::FnMessageHandler;
use bot::Message;
use bot::ResponseCallbacks;
use futures::StreamExt;
use handler::{history_window::history_window_pair, ping::ping, quote::Quote};
use std::{convert::TryFrom, env, error::Error, num::NonZeroU64};
use twilight_gateway::cluster::{Cluster, ShardScheme};
use twilight_http::Client;
use twilight_model::{
    channel::embed::{Embed as DiscordEmbed, EmbedAuthor, EmbedFooter, EmbedImage},
    gateway::Intents,
};

struct Callbacks {
    http: Client,
}

#[async_trait]
impl ResponseCallbacks for Callbacks {
    async fn send_message(&self, message: Message) -> Result<(), Box<dyn Error>> {
        let embeds: Vec<_> = message
            .embeds
            .into_iter()
            .map(|embed| {
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
            })
            .collect();

        self.http
            .create_message(NonZeroU64::try_from(message.channel_id).unwrap().into())
            .embeds(&embeds)?
            .content(&message.content)?
            .exec()
            .await?;

        Ok(())
    }

    async fn delete_message(&self, channel_id: u64, message_id: u64) -> Result<(), Box<dyn Error>> {
        self.http
            .delete_message(
                NonZeroU64::try_from(channel_id).unwrap().into(),
                NonZeroU64::try_from(message_id).unwrap().into(),
            )
            .exec()
            .await?;

        Ok(())
    }

    async fn fetch_message(&self, channel_id: u64, message_id: u64) -> Option<Message> {
        self.http
            .message(
                NonZeroU64::try_from(channel_id).unwrap().into(),
                NonZeroU64::try_from(message_id).unwrap().into(),
            )
            .exec()
            .await
            .ok()?
            .model()
            .await
            .ok()
            .map(|x| x.into())
    }

    async fn fetch_channel(&self, channel_id: u64) -> Option<Channel> {
        self.http
            .channel(NonZeroU64::try_from(channel_id).unwrap().into())
            .exec()
            .await
            .ok()?
            .model()
            .await
            .ok()
            .map(|x| x.into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = env::var("PETROLEUM_TOKEN")?;

    let (cluster, mut events) = Cluster::builder(&token, Intents::GUILD_MESSAGES)
        .shard_scheme(ShardScheme::Auto)
        .build()
        .await?;

    tokio::spawn(async move {
        cluster.up().await;
    });

    let http = Client::new(token);

    let mut handler = Bot::new(Callbacks { http });
    let (history_window, history_window_configurator) = history_window_pair();

    handler.on_message(Quote);
    handler.on_message(FnMessageHandler(ping));
    handler.on_message(history_window);
    handler.on_message(history_window_configurator);

    while let Some((_, event)) = events.next().await {
        handler.handle(event).await;
    }

    Ok(())
}
