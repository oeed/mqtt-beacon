/* use std::{fs, thread};

use mqtt_beacon::{config::Config, listener::Listener, mqtt_client::MqttClient};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let is_debug = args.get(1).map_or(false, |arg| arg == "debug");

  let config = fs::read_to_string("beacon-config.toml").expect("unable to read beacon-config.toml");
  let config: Config = toml::from_str(&config).expect("unable to parse beacon-config.toml");

  let (send_channel, mut connection, mut client) =
    MqttClient::with_config(config.mqtt_client).expect("unable to start mqtt client");

  let listener = Listener::with_config(config.listener);
  thread::spawn(move || listener.listen(send_channel, is_debug));

  // drive the event loop forever
  thread::spawn(move || connection.iter().for_each(drop));

  client.send_messages().unwrap();
}
 */


use std::error::Error;

use futures::stream::StreamExt;
use rumble::{
  api::{Central, CentralEvent, Peripheral, UUID},
  bluez::manager::Manager,
};

pub fn main() {
  let manager = Manager::new().unwrap();

  // get the first bluetooth adapter
  let adapters = manager.adapters().unwrap();
  let mut adapter = adapters.into_iter().nth(0).unwrap();

  // reset the adapter -- clears out any errant state
  adapter = manager.down(&adapter).unwrap();
  adapter = manager.up(&adapter).unwrap();

  // connect to the adapter
  let central = adapter.connect().unwrap();
  central.active(false);
  central.filter_duplicates(false);
  // start scanning for devices
  central.start_scan().unwrap();

  central.on_event(Box::new(|event| {
    println!("{:?}", event);
  }));


  // instead of waiting, you can use central.on_event to be notified of
  // new devices
  thread::sleep(Duration::from_secs(30));
}
