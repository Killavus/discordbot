use discord::model::{Message, MessageId, User};
use std::slice::Iter;
use chrono::DateTime;
use chrono::offset::FixedOffset;

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

  pub fn claim(&mut self, spawn_name: String, claim_message: Message) {
    self.0.push(ClaimedSpawn {
      claim_message,
      spawn_name,
    });
  }

  pub fn iter(&self) -> Iter<ClaimedSpawn> {
    self.0.iter()
  }
}
