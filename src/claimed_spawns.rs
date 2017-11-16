use discord::model::User;
use std::slice::Iter;

pub struct ClaimedSpawn {
   pub claimed_by: User,
   pub spawn_name: String
}

pub struct ClaimedSpawns(Vec<ClaimedSpawn>);

impl ClaimedSpawns {
  pub fn new() -> Self {
    ClaimedSpawns(Vec::new())
  }

  pub fn claim(&mut self, spawn_name: String, claimed_by: User) {
    self.0.push(ClaimedSpawn {
      claimed_by,
      spawn_name
    });
  }

  pub fn iter(&self) -> Iter<ClaimedSpawn> {
    self.0.iter()
  }
}


