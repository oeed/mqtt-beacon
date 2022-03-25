use btleplug::api::BDAddr;
use rumqttc::QoS;
use serde::Deserialize;

use crate::mqtt_client::{MqttPublish, PublishSender};

#[derive(Debug, Deserialize)]
pub struct BeaconConfig {
  /// The bluetooth address of the beacon
  beacon_address: BDAddr,
  /// Topic to broadcast to when the beacon is present
  topic: String,
  /// Payload to broadcast when present
  present_payload: String,
}

impl BeaconConfig {
  pub fn on_discovery(&self, discovered_address: BDAddr, send_channel: &PublishSender) {
    if discovered_address == self.beacon_address {
      log::debug!("Discovered known beacon: {}", &self.topic);

      // this is the beacon, publish
      if let Err(_) = send_channel.send(MqttPublish {
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
