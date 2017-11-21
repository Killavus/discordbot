use discord::Discord;
use discord::model::{Channel, ChannelId, ChannelType, PublicChannel, ServerId};
use errors::*;
use std::sync::Arc;
use discord_utils::clear_channel_messages;

pub struct BotChannels {
  info_channel: PublicChannel,
  interaction_channel: PublicChannel,
}

impl BotChannels {
  fn info_channel_name() -> &'static str {
    "respawns-claimed"
  }

  fn interaction_channel_name() -> &'static str {
    "respawns-bot"
  }

  pub fn interaction_channel_id(&self) -> ChannelId {
    self.interaction_channel.id
  }

  pub fn info_channel_id(&self) -> ChannelId {
    self.info_channel.id
  }
}

pub fn prepare_bot_channel(discord: Arc<Discord>, server_id: ServerId) -> Result<BotChannels> {
  let info_channel =
    bot_channel_for_server(discord.clone(), server_id, BotChannels::info_channel_name())?;
  let interaction_channel = bot_channel_for_server(
    discord.clone(),
    server_id,
    BotChannels::interaction_channel_name(),
  )?;

  rebuild_info_channel(discord.clone(), &info_channel)?;

  Ok(BotChannels {
    info_channel,
    interaction_channel,
  })
}

fn rebuild_info_channel(discord: Arc<Discord>, channel: &PublicChannel) -> Result<()> {
  clear_channel_messages(discord.clone(), channel.id)?;
  Ok(())
}

fn bot_channel_for_server(
  discord: Arc<Discord>,
  server_id: ServerId,
  channel_name: &str,
) -> Result<PublicChannel> {
  let channels = discord.get_server_channels(server_id).chain_err(|| {
    format!("Failed to get list of channels for server {}", server_id)
  })?;

  let maybe_channel = channels
    .into_iter()
    .find(|channel| channel.name == channel_name);

  match maybe_channel {
    Some(channel) => Ok(channel),
    None => {
      let new_channel = discord
        .create_channel(server_id, channel_name, ChannelType::Text)
        .chain_err(|| {
          format!(
            "Failed to create bot channel {} for server {}.",
            channel_name,
            server_id
          )
        })?;
      match new_channel {
        Channel::Public(channel) => return Ok(channel),
        _ => panic!("Wanted to create public channel but other channel type is returned."),
      }
    }
  }
}
