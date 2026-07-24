use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use crate::blocks::sysfs::SysFs;
use crate::blocks::BlockUpdate;
use crate::utils::get_device_dir;

const DEFAULT_INTERVAL: u64 = 5;

pub fn run(block_id: usize, sender: Sender<BlockUpdate>) {
    let Ok(Some(iface_path)) = get_device_dir("/sys/class/net", |name| name.starts_with("wlan"))
    else {
        eprintln!("net: could not get net device dir");
        return;
    };

    let iface = SysFs::new(iface_path);
    let iface_name = iface.get_basename().unwrap_or("iface");

    let mut operstate: String = String::from("down");
    let mut speed: String = String::from("-1");

    loop {
        let operstate_new = iface
            .read_string("operstate")
            .ok()
            .map_or(String::from("down"), |s| s.trim().to_owned());
        let speed_new = iface
            .read_string("speed")
            .ok()
            .map_or(String::from("-1"), |s| s.trim().to_owned());

        if operstate_new != operstate || speed_new != speed {
            operstate = operstate_new;
            speed = speed_new;

            let full_text: String;
            if operstate == "down" || speed == "-1" {
                full_text = format!("net: {} {}", iface_name, operstate);
            } else {
                full_text = format!("net: {} {} ({})", iface_name, operstate, speed);
            }

            if sender.send(BlockUpdate::new(block_id, &full_text)).is_err() {
                break;
            }
        }

        sleep(Duration::from_secs(DEFAULT_INTERVAL));
    }
}
