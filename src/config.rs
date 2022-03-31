use serde::Deserialize;

use crate::beacon::BeaconConfig;
use mqtt_garage::mqtt_client::MqttClientConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
  /// The MQTT configuration
  pub mqtt_client: MqttClientConfig,
  /// Configurations of the beacons
  pub beacons: Vec<BeaconConfig>,
}
