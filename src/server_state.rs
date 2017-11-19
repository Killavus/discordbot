use discord::Discord;
use discord::model::{Channel, ChannelType, PublicChannel, ServerId};
use errors::*;
use std::sync::Arc;

pub fn prepare_bot_channel(discord: Arc<Discord>, server_id: ServerId) -> Result<PublicChannel> {
  let channels = discord.get_server_channels(server_id).chain_err(|| {
    format!("Failed to get list of channels for server {}", server_id)
  })?;

  let maybe_channel = channels
    .into_iter()
    .find(|channel| channel.name == "respawns");

  match maybe_channel {
    Some(channel) => Ok(channel),
    None => {
      let new_channel = discord
        .create_channel(server_id, "respawns", ChannelType::Text)
        .chain_err(|| {
          format!("Failed to create bot channel for server {}.", server_id)
        })?;
      match new_channel {
        Channel::Public(channel) => return Ok(channel),
        _ => panic!("Wanted to create public channel but other channel type is returned."),
      }
    }
  }
}
