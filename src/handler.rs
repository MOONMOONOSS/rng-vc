use async_std::task;
use rand::prelude::*;
use voice::LockedAudio;

use std::{
  sync::Arc,
  time::Duration,
};

use serenity::{
  async_trait,
  client::{
    Context,
    EventHandler,
  },
  model::{
    channel::ChannelType,
    id::GuildId,
    prelude::Ready,
  },
  voice,
};

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, _: Ready) {
    println!("Bot is starting");

    let state = ctx.data.read().await;
    let config = Arc::clone(&state.get::<crate::Configuration>().expect("Where's muh config bro?"));
    let http = Arc::clone(&ctx.http);
    let cache = Arc::clone(&ctx.cache);
    let manager_lock = ctx.data.read().await.get::<crate::VoiceManager>().cloned().unwrap();

    let guild = GuildId(config.server_id);

    println!("Scheduled async task has started");

    loop {
      std::thread::sleep(Duration::from_secs(config.interval));

      // Get all voice channels in the guild
      // Must have permission to Connect and Speak without Push to Talk
      // At least one user must be connected as well
      if let Ok(channels) = guild.channels(&http).await {
        'chan_iter: for (chan_id, guild_chan) in channels.iter() {
          match &guild_chan.kind {
            ChannelType::Voice => {
              println!("{} is a voice channel", &chan_id);
              if let Ok(members) = guild_chan.members(&cache).await {
                if members.is_empty() {
                  continue 'chan_iter;
                }

                if rand::thread_rng().gen::<f64>() >= config.chance {
                  println!("The die is cast. Joining {}...", &chan_id);

                  // Join channel logic
                  let mut manager = manager_lock.lock().await;

                  if let Some(handler) = manager.join(&guild, chan_id) {
                    handler.join(*chan_id);
                    println!("Starting YouTube download...");
                    // let source = match voice::ytdl("https://youtube.com/watch?v=S7rM1zmCj1M").await {
                    let source = match voice::ytdl("https://www.youtube.com/watch?v=oNXzMBA9VU4").await {
                      Ok(source) => source,
                      Err(why) => {
                        println!("Bot fucked up fuck you YouTube {:#?}", why);

                        continue 'chan_iter;
                      },
                    };

                    print!("Playing audio");
                    let safe_audio: LockedAudio = handler.play_only(source);
                    {
                      let audio_lock = safe_audio.clone();
                      let audio = audio_lock.lock().await;

                      while audio.playing {
                        std::thread::sleep(Duration::from_millis(100));
                        print!(".");
                      }

                      println!("\nFinished playback");
                      manager.leave(&guild);
                    }
                  } else {
                    continue 'chan_iter;
                  }

                  // Leave for loop early
                  break 'chan_iter;
                }
              }
            }
            _ => {},
          };
        }
      }
    }
  }
}
