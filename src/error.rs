use std::sync::mpsc::RecvError;

use mqtt_garage::error::GarageError;
use thiserror::Error;
use tokio::task::JoinError;

pub type BeaconResult<T> = Result<T, BeaconError>;

#[derive(Debug, Error)]
pub enum BeaconError {
  #[error(transparent)]
  MQTTClient(#[from] rumqttc::ClientError),
  #[error(transparent)]
  MQTTConnection(#[from] rumqttc::ConnectionError),
  #[error(transparent)]
  JoinError(#[from] JoinError),
  #[error(transparent)]
  MpscRecv(#[from] RecvError),
  #[error(transparent)]
  Garage(#[from] GarageError),
  #[error(transparent)]
  Btleplug(#[from] btleplug::Error),
}
