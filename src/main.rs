use std::env;
mod bot;
use bot::BotConfig;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = BotConfig::new("config.toml").expect("failed");
    println!("config: {:?}", config)
}

extern crate test_macro;
use test_macro::my_macro;
use test_macro::show_streams;

#[my_macro]
fn testing() {
    println!("test")
}


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
//
//     //single shard atm but this is how to set a specific shard count
//     let shards = 1;
//     let scheme = ShardScheme::try_from((0..shards, shards))?;
//
//     let config = Config::builder(env::var("DISCORD_TOKEN")?)
//         .shard_scheme(scheme)
//         .intents(Some(
//             GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_BANS | GatewayIntents::GUILD_EMOJIS | GatewayIntents::GUILD_INVITES  |
//                 GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::DIRECT_MESSAGE_REACTIONS
//         ))
//         .build();
//
//     let cache_config = CacheConfig::builder()
//         .event_types(EventType::MEMBER_ADD
//                          | EventType::MEMBER_CHUNK
//                          | EventType::MEMBER_REMOVE
//                          | EventType::MEMBER_UPDATE,
//         )
//         .build();
//
//
//
//     let cluster = Cluster::new(config);
//
//     // command handling
//     let testgroup = CommandGroup();
//
//     cluster.up().await?;
//
//     let mut events = cluster.events().await;
//
//     while let Some((id, event)) = events.next().await {
//
//         println!("Shard: {} | Event: {:?}", id, event.event_type())
//     }
//
//     Ok(())
// }