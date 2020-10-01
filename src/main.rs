use std::{
  sync::Arc,
  thread::JoinHandle,
};

use serenity::{
  Client, 
  framework::StandardFramework,
  prelude::TypeMapKey,
};

mod config;
mod handler;

pub(crate) struct Configuration;

impl TypeMapKey for Configuration {
  type Value = Arc<config::Configuration>;
}

pub struct GameThread;

impl TypeMapKey for GameThread {
  type Value = JoinHandle<()>;
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
  }

  let why = match client.start().await {
    Err(it) => it,
    _ => return,
  };

  println!("Serenity error: {:?}", why);
}
