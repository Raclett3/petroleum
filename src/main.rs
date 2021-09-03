use futures::StreamExt;
use std::{env, error::Error};
use twilight_gateway::{
    cluster::{Cluster, ShardScheme},
    Event,
};
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

    while let Some((_, event)) = events.next().await {
        tokio::spawn(handle_event(event, http.clone()));
    }

    Ok(())
}

async fn handle_event(event: Event, http: Client) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content == "ping?" => {
            http.create_message(msg.channel_id)
                .content("pong!")?
                .exec()
                .await?;
        }
        _ => {}
    }

    Ok(())
}
