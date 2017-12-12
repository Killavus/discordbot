extern crate chrono;
extern crate csv;
extern crate discord;
#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod errors;
mod command;
mod spawns;
mod server_state;
mod discord_utils;

use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Sender};
use discord::{Connection, Discord, State};

use errors::*;
use command::Command;
use spawns::{claimed_spawn_embed, ClaimList, ClaimResult, SpawnList};
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
    let spawns = SpawnList::from_csv("./spawns.csv")?;
    let mut spawn_claims = ClaimList::new();

    let mut channel_pairs = HashMap::new();

    let (discord, connection, state) = initialize_discord(bot_key)?;
    let shared_state = Arc::new(RwLock::new(state));
    let shared_discord = Arc::new(discord);

    let (sender, receiver) = channel();

    thread::spawn(move || event_loop(connection, shared_state.clone(), sender.clone()));

    loop {
        let command = receiver.recv().unwrap();

        match command {
            Command::ClaimSpawn { spawn_msg, message } => {
                if !channel_pairs.contains_key(&message.channel_id) {
                    continue;
                }

                let interaction_channel_id = message.channel_id.clone();
                let info_channel_id = channel_pairs.get(&message.channel_id).unwrap();

                match spawn_claims.claim(&spawns, &spawn_msg, message.clone()) {
                    ClaimResult::NewClaim(spawn) => {
                        let claimed_spawn = spawn_claims.claim_by_code(&spawn.code).unwrap();

                        shared_discord
                            .send_embed(*info_channel_id, "", |builder| {
                                claimed_spawn_embed(claimed_spawn, builder)
                            })
                            .chain_err(|| "Failed to send message")?;
                    }
                    ClaimResult::UnknownSpawn => {
                        shared_discord
                            .send_message(
                                interaction_channel_id,
                                "Sorry, but I could not find the spawn you wanted to claim.",
                                "",
                                false,
                            )
                            .chain_err(|| "Failed to send message")?;
                    }
                    ClaimResult::ClaimedBefore(spawn) => {
                        let claimed_spawn = spawn_claims.claim_by_code(&spawn.code).unwrap();

                        if claimed_spawn.user().id == message.author.id {
                            shared_discord
                                .send_message(
                                    interaction_channel_id,
                                    "You've already claimed this spawn.",
                                    "",
                                    false,
                                )
                                .chain_err(|| "Failed to send message.")?;
                        } else {
                            shared_discord
                                .send_message(
                                    interaction_channel_id,
                                    &format!(
                                        "Sorry, but <@{}> claimed this spawn already. Maybe you should message him?",
                                        claimed_spawn.user().id
                                    ), 
                                    "",
                                    false
                                )
                                .chain_err(|| "Failed to send message")?;
                        }
                    }
                }
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
