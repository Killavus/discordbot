use std::convert::From;

pub enum Command {
  ClaimSpawn { spawn_name: String },
  Unknown,
}

impl From<String> for Command {
  fn from(value: String) -> Self {
    let cleaned_content = value.trim();

    if cleaned_content.starts_with("/claim") {
      let spawn_name = cleaned_content["/claim".len()..].trim();
      Command::ClaimSpawn {
        spawn_name: String::from(spawn_name),
      }
    }
    else {
      Command::Unknown
    }
  }
}
