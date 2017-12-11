use std::convert::From;
use discord::model::{Event, Message, PossibleServer, ServerId};

pub enum Command {
  ClaimSpawn { spawn_msg: String, message: Message },
  EstablishState { server_id: ServerId },
  Unknown,
}

impl Command {
  fn from_message(message: Message) -> Self {
    let content = String::from(message.content.trim());

    if content.starts_with("/claim") {
      let spawn_msg = content["/claim".len()..].trim();
      Command::ClaimSpawn {
        spawn_msg: String::from(spawn_msg),
        message,
      }
    } else {
      Command::Unknown
    }
  }
}

impl From<Event> for Command {
  fn from(value: Event) -> Self {
    match value {
      Event::MessageCreate(msg) => Command::from_message(msg),
      Event::ServerCreate(possible_server) => match possible_server {
        PossibleServer::Online(live_server) => Command::EstablishState {
          server_id: live_server.id,
        },
        PossibleServer::Offline(server_id) => Command::EstablishState { server_id },
      },
      _ => Command::Unknown,
    }
  }
}
