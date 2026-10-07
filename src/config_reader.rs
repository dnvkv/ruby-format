use std::path::Path;

pub struct Config;

pub trait ConfigReader {
    pub fn read_config<T: AsRef<Path>>(path: T) -> Option<Config>;
}
