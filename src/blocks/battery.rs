use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use crate::blocks::BlockUpdate;
use crate::utils::get_device_dir;

const DEFAULT_INTERVAL: u64 = 5;
const POWER_SUPPLY_DEVICES_DIR: &str = "/sys/class/power_supply";

struct Battery {
    device_path: PathBuf,
}

impl Battery {
    fn new(device_path: PathBuf) -> Battery {
        Battery {
            device_path: device_path,
        }
    }

    fn read_capacity(&self) -> Result<u8, Box<dyn Error>> {
        let capacity = read_to_string(self.device_path.join(PathBuf::from("capacity")))?;
        return Ok(capacity.trim().parse()?);
    }

    fn read_status(&self) -> Result<String, Box<dyn Error>> {
        return Ok(String::from(
            read_to_string(self.device_path.join(PathBuf::from("status")))?.trim(),
        ));
    }
}

pub fn run(block_id: usize, sender: Sender<BlockUpdate>) {
    let power_supply_path =
        match get_device_dir(POWER_SUPPLY_DEVICES_DIR, |name| name.starts_with("BAT")) {
            Ok(Some(value)) => value,
            _ => todo!("handle error"),
        };

    let battery = Battery::new(power_supply_path);

    loop {
        let capacity = match battery.read_capacity() {
            Ok(value) => value,
            Err(e) => todo!("{}", e),
        };
        let status = match battery.read_status() {
            Ok(value) => value,
            Err(_) => todo!("handle error"),
        };

        match sender.send(BlockUpdate::new(
            block_id,
            &format!("{} {}%", status, capacity),
        )) {
            Ok(()) => (),
            Err(_) => todo!("handle error"),
        };

        sleep(Duration::from_secs(DEFAULT_INTERVAL));
    }
}
