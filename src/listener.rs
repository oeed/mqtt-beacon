use std::sync::mpsc::Sender;

use btleplug::{
  api::{BDAddr, Central, CentralEvent, Manager as _, Peripheral, ScanFilter},
  platform::Manager,
};
use futures::stream::StreamExt;

use crate::error::BeaconResult;

#[derive(Debug)]
pub struct Listener;

impl Listener {
  pub async fn listen(tx: Sender<BDAddr>) -> BeaconResult<()> {
    log::debug!("Starting BLE listen...");
    let manager = Manager::new().await?;

    // get the first bluetooth adapter
    // connect to the adapter
    let adapters = manager.adapters().await.unwrap();
    let central = adapters.into_iter().nth(0).unwrap();

    let mut events = central.events().await?;

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;

    // Print based on whatever the event receiver outputs. Note that the event
    // receiver blocks, so in a real program, this should be run in its own
    // thread (not task, as this library does not yet use async channels).
    while let Some(event) = events.next().await {
      log::debug!("BLE event: {:?}", &event);
      match event {
        CentralEvent::DeviceDiscovered(id) | CentralEvent::DeviceUpdated(id) => {
          if let Ok(peripheral) = central.peripheral(&id).await {
            // we can ignore this error, if it fails the means something failed elsewhere and the program will soon end
            log::debug!("Address: {:?}", &peripheral.address());
            tx.send(peripheral.address()).ok();
          }
        }
        _ => (),
      }
    }

    Ok(())
  }
}
