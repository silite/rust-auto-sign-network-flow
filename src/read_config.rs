use std::{error::Error, fs::File, io::Read, path::Path};
use toml::from_str;

pub fn read_config<P, T>(path: P) -> Result<T, Box<dyn Error>>
where
    P: AsRef<Path>,
    T: serde::de::DeserializeOwned,
{
    let mut file = File::open(path)?;
    let mut config_string = String::new();
    file.read_to_string(&mut config_string)?;
    let config: T = from_str(&config_string)?;
    Ok(config)
}
