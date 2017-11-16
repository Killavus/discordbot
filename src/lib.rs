extern crate discord;
#[macro_use]
extern crate error_chain;

mod errors;
mod command;

use errors::*;
use discord::Discord;
use discord::State;
use discord::model::Event;
use command::Command;

pub fn run(bot_key: &str) -> Result<()> {
    let discord =
        Discord::from_bot_token(bot_key).chain_err(|| "Failed to initialize discord client.")?;

    let (mut connection, ready_state) = discord
        .connect()
        .chain_err(|| "Failed to initialize connection.")?;

    let mut state = State::new(ready_state);

    loop {
        if let Ok(event) = connection.recv_event() {
            state.update(&event);

            match event {
                Event::MessageCreate(message) => {
                    let command = Command::from(message.content.clone());
                    if let Command::ClaimSpawn { spawn_name } = command {
                        discord.send_message(
                            message.channel_id,
                            &format!("Spawn claimed: {}", spawn_name),
                            "",
                            false
                        ).chain_err(|| "Failed to send message")?;
                    }
                }
                _ => {}
            }
        } else {
            eprintln!("failed to receive an event.");
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
