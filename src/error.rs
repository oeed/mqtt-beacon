use thiserror::Error;

pub type BeaconResult<T> = Result<T, BeaconError>;

#[derive(Debug, Error)]
pub enum BeaconError {
  #[error(transparent)]
  MQTTClient(#[from] rumqttc::ClientError),
  #[error(transparent)]
  MQTTConnection(#[from] rumqttc::ConnectionError),
}
