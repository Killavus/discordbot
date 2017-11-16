extern crate discord;
#[macro_use]
extern crate error_chain;

mod errors;
mod command;
mod claimed_spawns;

use errors::*;
use discord::Discord;
use discord::State;
use discord::model::Event;
use command::Command;
use discord::Connection;


use claimed_spawns::ClaimedSpawns;

fn event_loop(discord: Discord, mut connection: Connection, mut state: State) -> Result<()> {
    let mut spawns = ClaimedSpawns::new();

    loop {
        if let Ok(event) = connection.recv_event() {
            state.update(&event);

            match event {
                Event::MessageCreate(message) => {
                    let command = Command::from(message.content.clone());
                    match command {
                        Command::ClaimSpawn { spawn_name } => {                            
                            discord.send_message(
                                message.channel_id,
                                &format!("Spawn claimed: {}", spawn_name),
                                "",
                                false
                            ).chain_err(|| "Failed to send message")?;
                            spawns.claim(spawn_name, message.author);
                        },
                        Command::ClaimedList => {
                            let mut content = String::from("Claimed spawns:\n");
                            spawns.iter().for_each(
                                |spawn| {
                                    content.push_str(
                                        &format!("> {} by **{}**\n", spawn.spawn_name, spawn.claimed_by.name)
                                    )
                                }
                            );

                            discord.send_message(
                                message.channel_id,
                                &content,
                                "",
                                false
                            ).chain_err(|| "Failed to send message")?;
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            eprintln!("Failed to receive an event.");
        }
    }
}

pub fn run(bot_key: &str) -> Result<()> {
    let discord =
        Discord::from_bot_token(bot_key).chain_err(|| "Failed to initialize Discord client.")?;

    let (connection, ready_state) = discord
        .connect()
        .chain_err(|| "Failed to initialize connection.")?;

    let state = State::new(ready_state);

    event_loop(discord, connection, state)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
