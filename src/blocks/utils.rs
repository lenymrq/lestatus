use std::{error::Error, fs::read_to_string, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct SysFs {
    device_path: PathBuf,
}

impl SysFs {
    pub fn new(device_path: PathBuf) -> SysFs {
        return SysFs {
            device_path: device_path,
        };
    }

    pub fn read_string(&self, path: &str) -> Result<String> {
        return Ok(String::from(
            read_to_string(self.device_path.join(PathBuf::from(path)))?.trim(),
        ));
    }

    pub fn read_u8(&self, path: &str) -> Result<u8> {
        let value =
            String::from(read_to_string(self.device_path.join(PathBuf::from(path)))?.trim());
        return Ok(value.parse()?);
    }

    pub fn get_basename(&self) -> Result<&str> {
        return match self.device_path.file_name() {
            Some(value) => match value.to_str() {
                Some(value) => Ok(value),
                None => Err(Box::from("could not convert basename to str")),
            },
            None => Err(Box::from("could not get basename")),
        };
    }
}
