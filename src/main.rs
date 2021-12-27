#[macro_use]
extern crate diesel;

mod bot;
mod handler;
mod models;
mod schema;

use async_trait::async_trait;
use bot::{Bot, Channel, FnMessageHandler, Message, ResponseCallbacks};
use diesel::{Connection, PgConnection};
use futures::StreamExt;
use handler::{
    history_window::{HistoryWindow, HistoryWindowConfigurator},
    ping::ping,
    quote::Quote,
};
use std::{convert::TryFrom, env, error::Error, num::NonZeroU64};
use twilight_gateway::cluster::{Cluster, ShardScheme};
use twilight_http::Client;
use twilight_model::gateway::Intents;

struct Callbacks {
    http: Client,
}

#[async_trait]
impl ResponseCallbacks for Callbacks {
    async fn send_message(&self, message: Message) -> Result<(), Box<dyn Error>> {
        let embeds: Vec<_> = message.embeds.into_iter().map(Into::into).collect();

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
    let database_url = std::env::var("DATABASE_URL")?;

    let (cluster, mut events) = Cluster::builder(&token, Intents::GUILD_MESSAGES)
        .shard_scheme(ShardScheme::Auto)
        .build()
        .await?;

    tokio::spawn(async move {
        cluster.up().await;
    });

    let http = Client::new(token);
    let db_conn = PgConnection::establish(&database_url).unwrap();

    let mut handler = Bot::new(Callbacks { http }, db_conn);

    handler.on_message(Quote);
    handler.on_message(FnMessageHandler(ping));
    handler.on_message(HistoryWindow);
    handler.on_message(HistoryWindowConfigurator);

    while let Some((_, event)) = events.next().await {
        handler.handle(event).await;
    }

    Ok(())
}
