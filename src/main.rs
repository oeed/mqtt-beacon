use std::{fs, thread, time::Duration};

use mqtt_beacon::{config::Config, error::BeaconError, listener::Listener};
use mqtt_garage::mqtt_client::MqttClient;

#[tokio::main]
async fn main() {
  env_logger::init();

  loop {
    let err = run().await;
    log::error!("Error occurred, restarting in 5 seconds: {:?}", err);
    // wait some time for the broker to come back online
    thread::sleep(Duration::from_secs(5));
  }
}

async fn run() -> BeaconError {
  let config = fs::read_to_string("beacon-config.toml").expect("unable to read beacon-config.toml");
  let config: Config = toml::from_str(&config).expect("unable to parse beacon-config.toml");

  let (send_channel, mut client) = MqttClient::with_config("mqtt-beacon", config.mqtt_client);

  thread::spawn(move || {
    let rx = Listener::listen();
    loop {
      if let Ok(address) = rx.recv() {
        log::debug!("Discovered: {:?}", &address);
        for beacon_config in &config.beacons {
          beacon_config.on_discovery(address, &send_channel);
        }
      } else {
        return;
      }
    }
  });

  client.announce().await.expect("failed to announce client");

  let mut receiver = client.receiver;
  let receive = tokio::spawn(async move { receiver.receive_messages().await.unwrap() });

  let mut sender = client.sender;
  let send = tokio::spawn(async move { sender.send_messages().await.unwrap() });

  // the two tasks will only end if an error occurs (most likely MQTT broker disconnection)
  tokio::try_join!(receive, send).unwrap_err().into()
}
