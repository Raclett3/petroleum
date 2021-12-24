mod bot;
mod handler;

use async_trait::async_trait;
use bot::Bot;
use bot::FnMessageHandler;
use bot::Message;
use bot::ResponseCallbacks;
use futures::StreamExt;
use handler::{history_window::history_window_pair, ping::ping};
use std::{env, error::Error};
use twilight_gateway::cluster::{Cluster, ShardScheme};
use twilight_http::Client;
use twilight_model::gateway::Intents;

struct Callbacks {
    http: Client,
}

#[async_trait]
impl ResponseCallbacks for Callbacks {
    async fn send_message(&self, message: Message) -> Result<(), Box<dyn Error>> {
        self.http
            .create_message(message.channel_id.into())
            .content(&message.content)?
            .exec()
            .await?;

        Ok(())
    }

    async fn delete_message(&self, channel_id: u64, message_id: u64) -> Result<(), Box<dyn Error>> {
        self.http
            .delete_message(channel_id.into(), message_id.into())
            .exec()
            .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = env::var("PETROLEUM_TOKEN")?;

    let (cluster, mut events) = Cluster::builder(&token, Intents::GUILD_MESSAGES)
        .shard_scheme(ShardScheme::Auto)
        .build()
        .await?;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    let http = Client::new(token);

    let mut handler = Bot::new(Callbacks { http: http.clone() });
    let (history_window, history_window_configurator) = history_window_pair();
    handler.on_message(FnMessageHandler(ping));
    handler.on_message(history_window);
    handler.on_message(history_window_configurator);

    while let Some((_, event)) = events.next().await {
        handler.handle(event).await;
    }

    Ok(())
}
