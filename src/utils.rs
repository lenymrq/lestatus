use std::fs::read_dir;
use std::io::Result;
use std::path::PathBuf;

pub fn get_device_dir<F>(devices_dir: &str, matches: F) -> Result<Option<PathBuf>>
where
    F: Fn(&str) -> bool,
{
    for entry in read_dir(devices_dir)? {
        let entry = entry?;
        if !entry.path().is_dir() {
            continue;
        }

        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        if matches(&name) {
            eprintln!("sysfs: found {}", entry.path().display());
            return Ok(Some(entry.path()));
        }
    }

    return Ok(None);
}
