use std::{fs, thread, time::Duration};

use mqtt_beacon::{config::Config, listener::Listener, mqtt_client::MqttClient};

fn main() {
  loop {
    let err = run();
    log::error!("Error occurred, restarting in 5 seconds: {:?}", err);
    // wait some time for the broker to come back online
    thread::sleep(Duration::from_secs(5));
  }
}

fn run() {
  let config = fs::read_to_string("beacon-config.toml").expect("unable to read beacon-config.toml");
  let config: Config = toml::from_str(&config).expect("unable to parse beacon-config.toml");

  let (send_channel, mut connection, mut client) =
    MqttClient::with_config(config.mqtt_client).expect("unable to start mqtt client");

  thread::spawn(move || {
    let rx = Listener::listen();
    loop {
      if let Ok(address) = rx.recv() {
        for beacon_config in &config.beacons {
          beacon_config.on_discovery(address, &send_channel);
        }
      }
      else {
        return;
      }
    }
  });

  // drive the event loop forever
  thread::spawn(move || connection.iter().for_each(drop));

  client.send_messages().unwrap();
}
