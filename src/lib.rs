extern crate discord;
#[macro_use]
extern crate error_chain;

mod errors;

use errors::*;

pub fn run(_bot_key: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
