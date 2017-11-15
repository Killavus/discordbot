extern crate discord;
#[macro_use]
extern crate error_chain;

mod errors;

use errors::*;
use discord::Discord;
use discord::State;

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
