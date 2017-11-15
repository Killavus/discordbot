extern crate discordbot;
extern crate dotenv;

use std::env;
use std::process;
use discordbot::run;

fn main() {
    dotenv::dotenv().ok();

    let bot_token = env::var("BOT_KEY").expect("You need to provide BOT_KEY environment variable.");

    if let Err(ref err) = run(&bot_token) {
        eprintln!("{}", err);
        for cause in err.iter().skip(1) {
            eprintln!("caused by: {}", cause);
        }

        if let Some(backtrace) = err.backtrace() {
            eprintln!("backtrace: {:?}", backtrace);
        }

        process::exit(1);
    }
}
