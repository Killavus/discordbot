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
use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

fn event_loop(discord: Discord, mut connection: Connection, shared_state: Arc<RwLock<State>>, sender: Sender<Command>) -> Result<()> {
    let mut spawns = ClaimedSpawns::new();

    loop {
        if let Ok(event) = connection.recv_event() {
            {
                let mut state = shared_state.write().unwrap();
                state.update(&event);
            }
            
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
    let (discord, connection, state) = initialize_discord(bot_key)?;
    
    let shared_state = Arc::new(RwLock::new(state));
    let (sender, receiver) = channel();

    let loop_thread = thread::spawn(move || {
        event_loop(discord, connection, shared_state.clone(), sender.clone())
    });

    loop_thread.join().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
