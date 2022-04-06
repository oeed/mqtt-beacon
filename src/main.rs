use std::fs;

use mqtt_beacon::{
  config::Config,
  error::{BeaconError, BeaconResult},
  listener::Listener,
};
use mqtt_garage::mqtt_client::MqttClient;

#[tokio::main]
async fn main() -> BeaconResult<()> {
  env_logger::init();

  Err(run().await)
}

async fn run() -> BeaconError {
  let config = fs::read_to_string("beacon-config.toml").expect("unable to read beacon-config.toml");
  let config: Config = toml::from_str(&config).expect("unable to parse beacon-config.toml");

  let (send_channel, mut client) = MqttClient::with_config("mqtt-beacon", config.mqtt_client);

  let listen = tokio::spawn(async move {
    let rx = Listener::listen()?;
    loop {
      match rx.recv() {
        Ok(address) => {
          log::debug!("Discovered: {:?}", &address);
          for beacon_config in &config.beacons {
            beacon_config.on_discovery(address, &send_channel);
          }
        }
        Err(err) => return Err::<(), BeaconError>(err.into()),
      }
    }
  });

  client.announce().await.expect("failed to announce client");

  let mut receiver = client.receiver;
  let receive = tokio::spawn(async move { receiver.receive_messages().await.unwrap() });

  let mut sender = client.sender;
  let send = tokio::spawn(async move { sender.send_messages().await.unwrap() });

  // the two tasks will only end if an error occurs (most likely MQTT broker disconnection)
  tokio::try_join!(receive, send, listen).unwrap_err().into()
}
