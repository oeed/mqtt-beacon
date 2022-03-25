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

use btleplug::{
  api::{bleuuid::BleUuid, Central, CentralEvent, Manager as _, ScanFilter},
  platform::{Adapter, Manager},
};
use futures::stream::StreamExt;

async fn get_central(manager: &Manager) -> Adapter {
  let adapters = manager.adapters().await.unwrap();
  adapters.into_iter().nth(0).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let manager = Manager::new().await?;

  // get the first bluetooth adapter
  // connect to the adapter
  let central = get_central(&manager).await;

  // Each adapter has an event stream, we fetch via events(),
  // simplifying the type, this will return what is essentially a
  // Future<Result<Stream<Item=CentralEvent>>>.
  let mut events = central.events().await?;

  // start scanning for devices
  central.start_scan(ScanFilter::default()).await?;

  // Print based on whatever the event receiver outputs. Note that the event
  // receiver blocks, so in a real program, this should be run in its own
  // thread (not task, as this library does not yet use async channels).
  while let Some(event) = events.next().await {
    match event {
      CentralEvent::DeviceDiscovered(id) => {
        println!("DeviceDiscovered: {:?}", id);
      }
      CentralEvent::DeviceConnected(id) => {
        println!("DeviceConnected: {:?}", id);
      }
      CentralEvent::DeviceDisconnected(id) => {
        println!("DeviceDisconnected: {:?}", id);
      }
      CentralEvent::ManufacturerDataAdvertisement { id, manufacturer_data } => {
        println!("ManufacturerDataAdvertisement: {:?}, {:?}", id, manufacturer_data);
      }
      CentralEvent::ServiceDataAdvertisement { id, service_data } => {
        println!("ServiceDataAdvertisement: {:?}, {:?}", id, service_data);
      }
      CentralEvent::ServicesAdvertisement { id, services } => {
        let services: Vec<String> = services.into_iter().map(|s| s.to_short_string()).collect();
        println!("ServicesAdvertisement: {:?}, {:?}", id, services);
      }
      _ => {}
    }
  }
  Ok(())
}
