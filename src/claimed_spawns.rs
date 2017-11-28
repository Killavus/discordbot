use discord::model::{Message, MessageId, User};
use std::slice::Iter;
use chrono::DateTime;
use chrono::offset::FixedOffset;
use discord::builders::EmbedBuilder;

pub struct ClaimedSpawn {
  claim_message: Message,
  pub spawn_name: String,
}

impl ClaimedSpawn {
  pub fn claimed_at(&self) -> &DateTime<FixedOffset> {
    &self.claim_message.timestamp
  }

  pub fn user(&self) -> &User {
    &self.claim_message.author
  }

  pub fn message_id(&self) -> MessageId {
    self.claim_message.id
  }
}

pub struct ClaimedSpawns(Vec<ClaimedSpawn>);

impl ClaimedSpawns {
  pub fn new() -> Self {
    ClaimedSpawns(Vec::new())
  }

  pub fn claim(&mut self, spawn_name: String, claim_message: Message) -> &ClaimedSpawn {
    self.0.push(ClaimedSpawn {
      claim_message,
      spawn_name,
    });

    self.iter().last().unwrap()
  }

  pub fn iter(&self) -> Iter<ClaimedSpawn> {
    self.0.iter()
  }
}

pub fn claimed_spawn_embed(claimed_spawn: &ClaimedSpawn, builder: EmbedBuilder) -> EmbedBuilder {
  let description = format!(
    "Spawn has been claimed on {}",
    claimed_spawn.claimed_at().format("%e %B, %H:%M")
  );

  builder
    .title(&claimed_spawn.spawn_name)
    .description(&description)
    .fields(|fbuilder| {
      fbuilder
        .field(
          "Claimed by",
          &format!("<@{}>", claimed_spawn.user().id),
          true,
        )
        .field("Last update", "N/A", true)
    })
}
