use discord::{Discord, GetMessages};
use discord::model::{ChannelId, MessageId};
use errors::*;

pub fn clear_channel_messages<T: AsRef<Discord>>(discord: T, channel_id: ChannelId) -> Result<()> {
  let discord_client = discord.as_ref();
  let mut what = GetMessages::MostRecent;
  let mut messages = Vec::new();
  loop {
    let message_batch = discord_client
      .get_messages(channel_id, what, Some(50))
      .chain_err(|| "Failed to get the message batch")?;

    if message_batch.len() > 0 {
      what = GetMessages::Before(message_batch.iter().last().unwrap().id);
      messages.extend(message_batch);
    } else {
      break;
    }
  }

  for messages_chunk in messages.chunks(100) {
    let message_ids: Vec<MessageId> = messages_chunk.iter().map(|message| message.id).collect();

    discord_client
      .delete_messages(channel_id, &message_ids)
      .chain_err(|| "Failed to delete messages.")?;
  }

  Ok(())
}
