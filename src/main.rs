mod handler;

use futures::StreamExt;
use handler::ping::ping;
use handler::Bot;
use std::{env, error::Error};
use twilight_gateway::cluster::{Cluster, ShardScheme};
use twilight_http::Client;
use twilight_model::gateway::Intents;

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

    let mut handler = Bot::new();
    handler.on_message(ping);

    while let Some((_, event)) = events.next().await {
        let messages = handler.handle(event);
        for message in messages {
            http.create_message(message.channel_id.into())
                .content(&message.content)?
                .exec()
                .await?;
        }
    }

    Ok(())
}
