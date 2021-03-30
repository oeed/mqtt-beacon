use std::{error, fmt};

pub type BeaconResult<T> = Result<T, BeaconError>;

#[derive(Debug)]
pub enum BeaconError {
  MQTTClient(rumqttc::ClientError),
  MQTTConnection(rumqttc::ConnectionError),
}


impl fmt::Display for BeaconError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      BeaconError::MQTTClient(ref e) => e.fmt(f),
      BeaconError::MQTTConnection(ref e) => e.fmt(f),
    }
  }
}

impl error::Error for BeaconError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match *self {
      BeaconError::MQTTClient(ref e) => Some(e),
      BeaconError::MQTTConnection(ref e) => Some(e),
    }
  }
}

impl From<rumqttc::ClientError> for BeaconError {
  fn from(err: rumqttc::ClientError) -> BeaconError {
    BeaconError::MQTTClient(err)
  }
}


impl From<rumqttc::ConnectionError> for BeaconError {
  fn from(err: rumqttc::ConnectionError) -> BeaconError {
    BeaconError::MQTTConnection(err)
  }
}
