use btleplug::api::BDAddr;
use mqtt_garage::mqtt_client::{sender::PublishSender, MqttPublish};
use rumqttc::QoS;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct BeaconConfig {
  /// The bluetooth address of the beacon
  beacon_address: BDAddr,
  /// Topic to broadcast to when the beacon is present
  topic: String,
  /// The device ID to use
  device_id: String,
}

#[derive(Debug, Serialize)]
struct PresencePayload<'a> {
  #[serde(rename = "id")]
  device_id: &'a str,
  name: &'a str,
  distance: f32,
}

impl BeaconConfig {
  pub fn on_discovery(&self, discovered_address: BDAddr, distance: Option<f32>, send_channel: &PublishSender) {
    if discovered_address == self.beacon_address {
      log::debug!("Discovered known beacon: {}", &self.topic);

      let payload = PresencePayload {
        device_id: &self.device_id,
        name: &self.device_id,
        distance: distance.unwrap_or(1.0),
      };

      // this is the beacon, publish
      if let Err(_) = send_channel.send(MqttPublish {
        topic: self.topic.clone(),
        qos: QoS::AtLeastOnce,
        retain: false,
        payload: serde_json::to_string(&payload).expect("failed serialization"),
      }) {
        // channel has ended
        return;
      }
    }
  }
}
