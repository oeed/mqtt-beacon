use btleplug::api::BDAddr;
#[cfg(target_os = "linux")]
use rumble::{
  api::{Central, Peripheral, UUID},
  bluez::manager::Manager,
};

#[derive(Debug)]
pub struct Listener;

impl Listener {
  #[cfg(target_os = "linux")]
  pub fn listen(on_beacon: impl Fn(BDAddr) -> ()) {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    // connect to the adapter
    let central = adapter.connect().unwrap();
    central.active(false);
    central.filter_duplicates(false);
    // start scanning for devices
    central.start_scan().unwrap();

    central.on_event(Box::new(|event| match event {
      CentralEvent::DeviceDiscovered(address) | CentralEvent::DeviceUpdated(address) => {
        on_beacon(BDAddr::from(address.address));
      }
      _ => (),
    }));
  }

  #[cfg(target_os = "macos")]
  pub fn listen(on_beacon: impl Fn(BDAddr) -> ()) {
    loop {
      // just demo/debug
      on_beacon("01:23:45:67:89:AB".parse().unwrap());
      std::thread::sleep(std::time::Duration::from_secs(2));
    }
  }
}
