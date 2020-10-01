use std::{
  marker::PhantomData,
  sync::Arc,
};

use async_std::task::JoinHandle;
use serenity::{
  Client,
  client::bridge::voice::ClientVoiceManager,
  framework::StandardFramework,
  prelude::{
    Mutex,
    TypeMapKey,
  },
};

mod config;
mod handler;

pub(crate) struct Configuration;

impl TypeMapKey for Configuration {
  type Value = Arc<config::Configuration>;
}

pub(crate) struct GameThread<T> {
  phantom: PhantomData<T>,
}

impl<T: 'static + std::marker::Send> TypeMapKey for GameThread<T> {
  type Value = &'static JoinHandle<T>;
}

pub(crate) struct VoiceManager;

impl TypeMapKey for VoiceManager {
  type Value = Arc<Mutex<ClientVoiceManager>>;
}

#[tokio::main]
async fn main() {
  let config = Arc::new(config::Configuration::read().expect("Unable to open configuration"));

  let mut client = Client::new(&config.token)
    .event_handler(handler::Handler)
    .framework(StandardFramework::new())
    .await
    .expect("Error creating Discord client");

  {
    let mut data = client.data.write().await;

    data.insert::<Configuration>(Arc::clone(&config));
    data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
  }

  let why = match client.start().await {
    Err(it) => it,
    _ => return,
  };

  println!("Serenity error: {:?}", why);
}
