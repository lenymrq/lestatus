// files to check
// - /sys/class/net/<iface>/operstate
// - /sys/class/net/<iface>/speed

use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use crate::blocks::utils::SysFs;
use crate::blocks::BlockUpdate;
use crate::utils::get_device_dir;

const DEFAULT_INTERVAL: u64 = 5;

pub fn run(block_id: usize, sender: Sender<BlockUpdate>) {
    let iface_path = match get_device_dir("/sys/class/net", |name| name.starts_with("wlan")) {
        Ok(Some(value)) => value,
        _ => todo!("handle error"),
    };

    // TODO: better way to retrieve iface name
    let iface = SysFs::new(iface_path.clone());
    let iface_name = match iface.get_basename() {
        Ok(value) => value,
        Err(_) => todo!("handle error"),
    };

    loop {
        let operstate = match iface.read_string("operstate") {
            Ok(value) => value,
            Err(_) => todo!("handle error"),
        };
        // let speed = match iface.read_string("speed") {
        //     Ok(value) => value,
        //     Err(_) => todo!("handle error"),
        // };

        match sender.send(BlockUpdate::new(
            block_id,
            &format!("{} {}", iface_name, operstate),
        )) {
            Ok(()) => (),
            Err(_) => todo!("handle error"),
        }

        sleep(Duration::from_secs(DEFAULT_INTERVAL));
    }
}
