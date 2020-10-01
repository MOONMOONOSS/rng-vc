use std::{sync::Arc, time::Duration};

use serenity::{
  async_trait,
  client::{
    Context,
    EventHandler,
  },
  model::prelude::Ready,
};

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, _: Ready) {
    println!("Bot is ready. Poggers.");
    let mut state = ctx.data.write().await;
    let config = Arc::clone(&state.get::<crate::Configuration>().expect("Where's muh config bro?"));
    let http = Arc::clone(&ctx.http);

    state.insert::<crate::GameThread>(std::thread::spawn(move || {
      loop {
        std::thread::sleep(Duration::from_secs(config.interval));

        // Get all voice channels in the guild
        // Must have permission to Connect and Speak without Push to Talk
        // At least one user must be connected as well
      }
    }));
  }
}
