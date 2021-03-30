use std::str::FromStr;

use btleplug::api::{BDAddr, Central, CentralEvent};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::manager::Manager;
use rumqttc::QoS;
use serde::Deserialize;

use crate::mqtt_client::{MqttPublish, PublishSender};

#[derive(Debug, Deserialize)]
pub struct ListenerConfig {
  /// The bluetooth address of the beacon
  beacon_address: String,

  /// Topic to broadcast to when the beacon is present
  topic: String,
  /// Payload to broadcast when present
  present_payload: String,
}

#[derive(Debug)]
pub struct Listener {
  /// The bluetooth address of the beacon
  beacon_address: BDAddr,

  /// Topic to broadcast to when the beacon is present
  topic: String,
  /// Payload to broadcast when present
  present_payload: String,
}

impl Listener {
  pub fn with_config(config: ListenerConfig) -> Self {
    Listener {
      beacon_address: BDAddr::from_str(&config.beacon_address).expect("invalid beacon_address"),
      topic: config.topic,
      present_payload: config.present_payload,
    }
  }

  pub fn listen(self, channel: PublishSender) {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    central.start_scan().unwrap();
    // instead of waiting, you can use central.event_receiver() to fetch a channel and
    // be notified of new devices

    let receiver = central.event_receiver().unwrap();
    while let Ok(message) = receiver.recv() {
      use CentralEvent::*;
      let address = match message {
        DeviceDiscovered(address) | ServiceDataAdvertisement { address, .. } => Some(address),
        _ => None,
      };

      if let Some(address) = address {
        if address == self.beacon_address {
          // this is our beacon
          if let Err(_) = channel.send(MqttPublish {
            topic: self.topic.clone(),
            qos: QoS::AtLeastOnce,
            retain: false,
            payload: self.present_payload.clone(),
          }) {
            // channel has ended
            return;
          }
        }
      }
    }
  }
}
