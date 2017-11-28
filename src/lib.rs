extern crate chrono;
extern crate discord;
#[macro_use]
extern crate error_chain;

mod errors;
mod command;
mod claimed_spawns;
mod server_state;
mod discord_utils;

use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Sender};
use discord::{Connection, Discord, State};

use errors::*;
use command::Command;
use claimed_spawns::ClaimedSpawns;
use std::collections::HashMap;

fn event_loop(
    mut connection: Connection,
    shared_state: Arc<RwLock<State>>,
    sender: Sender<Command>,
) -> Result<()> {
    loop {
        if let Ok(event) = connection.recv_event() {
            {
                let mut state = shared_state.write().unwrap();
                state.update(&event);
            }

            let command = Command::from(event);
            match command {
                Command::Unknown => (),
                _ => sender.send(command).unwrap(),
            }
        } else {
            eprintln!("Failed to receive an event.");
        }
    }
}

fn initialize_discord(bot_key: &str) -> Result<(Discord, Connection, State)> {
    let discord =
        Discord::from_bot_token(bot_key).chain_err(|| "Failed to initialize Discord client.")?;

    let (connection, ready_state) = discord
        .connect()
        .chain_err(|| "Failed to initialize connection.")?;

    let state = State::new(ready_state);

    Ok((discord, connection, state))
}

pub fn run(bot_key: &str) -> Result<()> {
    let mut spawns = ClaimedSpawns::new();
    let mut channel_pairs = HashMap::new();

    let (discord, connection, state) = initialize_discord(bot_key)?;

    let shared_state = Arc::new(RwLock::new(state));
    let shared_discord = Arc::new(discord);

    let (sender, receiver) = channel();

    thread::spawn(move || {
        event_loop(connection, shared_state.clone(), sender.clone())
    });

    loop {
        let command = receiver.recv().unwrap();

        match command {
            Command::ClaimSpawn {
                spawn_name,
                message,
            } => {
                if !channel_pairs.contains_key(&message.channel_id) {
                    continue;
                }

                let info_channel_id = channel_pairs.get(&message.channel_id).unwrap();

                shared_discord
                    .send_message(
                        *info_channel_id,
                        &format!("Spawn claimed: {}", spawn_name),
                        "",
                        false,
                    )
                    .chain_err(|| "Failed to send message")?;
                spawns.claim(spawn_name, message);
            }
            Command::ClaimedList { message } => {
                if !channel_pairs.contains_key(&message.channel_id) {
                    continue;
                }

                let mut content = String::from("Claimed spawns:\n");
                spawns.iter().for_each(|spawn| {
                    content.push_str(&format!(
                        "> {} by **{}**\n",
                        spawn.spawn_name,
                        spawn.user().name
                    ))
                });

                shared_discord
                    .send_message(message.channel_id, &content, "", false)
                    .chain_err(|| "Failed to send message")?;
            }
            Command::EstablishState { server_id } => {
                let bot_channels =
                    server_state::prepare_bot_channel(shared_discord.clone(), server_id)?;

                channel_pairs.insert(
                    bot_channels.interaction_channel_id(),
                    bot_channels.info_channel_id(),
                );
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
