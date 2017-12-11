use discord::model::{Message, User};
use chrono::DateTime;
use chrono::offset::FixedOffset;
use discord::builders::EmbedBuilder;
use std::rc::Rc;
use errors::*;
use std::fs::File;
use std::io::BufReader;
use csv;

type SpawnItem = Rc<Box<Spawn>>;

pub struct SpawnList(Vec<SpawnItem>);
pub struct ClaimList(Vec<SpawnClaim>);

#[derive(Debug, Deserialize)]
pub struct Spawn {
  name: String,
  code: String,
  category: String,
}

pub struct SpawnClaim {
  message: Message,
  spawn: SpawnItem,
}

impl SpawnList {
  pub fn from_csv(file_name: &str) -> Result<Self> {
    let file = File::open(file_name).chain_err(|| "Failed to open spawns file.")?;
    let raw_reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(raw_reader);

    let mut list = Vec::new();
    for spawn_entry in reader.deserialize() {
      let spawn: Spawn = spawn_entry.chain_err(|| "Malformed spawn on the list")?;
      list.push(Rc::new(Box::new(spawn)));
    }

    Ok(SpawnList(list))
  }

  pub fn find_from_msg(&self, spawn_msg: &str) -> Option<SpawnItem> {
    self
      .0
      .iter()
      .find(|item| {
        item.code == spawn_msg || item.name.to_lowercase() == spawn_msg.to_lowercase()
      })
      .map(|item| item.clone())
  }

  pub fn find(&self, code: &str) -> Option<SpawnItem> {
    self
      .0
      .iter()
      .find(|item| item.code == code)
      .map(|item| item.clone())
  }
}

impl ClaimList {
  pub fn new() -> Self {
    ClaimList(Vec::new())
  }

  pub fn claim(
    &mut self,
    spawn_list: &SpawnList,
    spawn_msg: &str,
    message: Message,
  ) -> Option<&SpawnClaim> {
    match spawn_list.find_from_msg(spawn_msg) {
      Some(spawn) => {
        self.0.push(SpawnClaim {
          message,
          spawn: spawn.clone(),
        });

        Some(self.0.iter().last().unwrap())
      }
      None => None,
    }
  }
}

impl SpawnClaim {
  pub fn claimed_at(&self) -> &DateTime<FixedOffset> {
    &self.message.timestamp
  }

  pub fn user(&self) -> &User {
    &self.message.author
  }
}

pub fn claimed_spawn_embed(claimed_spawn: &SpawnClaim, builder: EmbedBuilder) -> EmbedBuilder {
  let description = format!(
    "Spawn has been claimed on {}",
    claimed_spawn.claimed_at().format("%e %B, %H:%M")
  );

  builder
    .title(&claimed_spawn.spawn.name)
    .description(&description)
    .fields(|fbuilder| {
      fbuilder
        .field(
          "Claimed by",
          &format!("<@{}>", claimed_spawn.user().id),
          true,
        )
        .field(
          "Last update",
          &format!("{}", claimed_spawn.claimed_at().format("%e %B, %H:%M")),
          true,
        )
    })
}
