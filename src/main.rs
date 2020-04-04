use twilight_gateway::cluster::{config::ShardScheme, Cluster, Config};
use twilight_model::gateway::GatewayIntents;

use futures::StreamExt;
use std::{error::Error, env};
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {


    // This is also the default.
    let scheme = ShardScheme::try_from((0..1, 1))?;

    let config = Config::builder(env::var("DISCORD_TOKEN")?)
        .shard_scheme(scheme)
        .intents(Some(
            GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_BANS | GatewayIntents::GUILD_EMOJIS | GatewayIntents::GUILD_INVITES  |
                GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::DIRECT_MESSAGE_REACTIONS
        ))
        .build();

    let cluster = Cluster::new(config);

    cluster.up().await?;

    let mut events = cluster.events().await;

    while let Some((id, event)) = events.next().await {
        println!("Shard: {} | Event: {:?}", id, event.event_type())
    }

    Ok(())
}