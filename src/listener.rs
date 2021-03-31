use btleplug::api::{bleuuid::uuid_from_u16, Central, CentralEvent};
#[cfg(target_os = "linux")]
use btleplug::bluez::manager::Manager;
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::manager::Manager;
use rumqttc::QoS;
use serde::Deserialize;
use uuid::Uuid;

use crate::mqtt_client::{MqttPublish, PublishSender};

#[derive(Debug, Deserialize)]
pub struct ListenerConfig {
  /// The bluetooth address of the beacon
  beacon_uuid: u16,
  /// The service data the beacon emits
  service_data: Vec<u8>,
  /// Topic to broadcast to when the beacon is present
  topic: String,
  /// Payload to broadcast when present
  present_payload: String,
}

#[derive(Debug)]
pub struct Listener {
  /// The bluetooth address of the beacon
  beacon_uuid: Uuid,
  /// The service data the beacon emits
  service_data: Vec<u8>,
  /// Topic to broadcast to when the beacon is present
  topic: String,
  /// Payload to broadcast when present
  present_payload: String,
}

impl Listener {
  pub fn with_config(config: ListenerConfig) -> Self {
    Listener {
      beacon_uuid: uuid_from_u16(config.beacon_uuid),
      service_data: config.service_data,
      topic: config.topic,
      present_payload: config.present_payload,
    }
  }

  pub fn listen(self, channel: PublishSender, is_debug: bool) {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let central = adapters.into_iter().nth(0).unwrap();
    central.filter_duplicates(false);
    central.active(false);

    // start scanning for devices
    central.start_scan().unwrap();
    // instead of waiting, you can use central.event_receiver() to fetch a channel and
    // be notified of new devices

    let receiver = central.event_receiver().unwrap();
    while let Ok(message) = receiver.recv() {
      use CentralEvent::*;
      if is_debug {
        println!("{:?}", &message);
      }
      let service = match message {
        ServiceDataAdvertisement { service, data, .. } => Some((service, data)),
        _ => None,
      };

      if let Some((uuid, service_data)) = service {
        if uuid == self.beacon_uuid {
          // this is the right uuid, but it could be another beacon
          if service_data == self.service_data {
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
          else if is_debug {
            println!("UUID matched, but service data didn't");
          }
        }
      }
    }
  }
}
