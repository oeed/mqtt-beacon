use serde::Deserialize;

use crate::{listener::Listener, mqtt_client::MqttClientConfig};

#[derive(Debug, Deserialize)]
pub struct Config {
  /// The MQTT configuration
  pub mqtt_client: MqttClientConfig,
  /// Configuration of the beacon
  pub listener: Listener,
}
