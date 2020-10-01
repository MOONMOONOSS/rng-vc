use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Configuration {
  pub(crate) token: String,
  pub(crate) server_id: u64,
  pub(crate) interval: u64, // Time interval per attempt in seconds
  pub(crate) chance: f64, // Chance per attempt in decimal (eg 90% is equal to 0.9)
}

impl Configuration {
  pub(crate) fn read() -> Result<Self, Box<dyn Error>> {
    let file = File::open("./config.yaml")?;
    let reader = BufReader::new(file);

    Ok(serde_yaml::from_reader(reader)?)
  }
}
