use super::global_config::GlobalConfig;

pub trait ConfigReader {
    fn read_global(&self) -> &GlobalConfig;
}

pub struct PlainReader {
    global_config: GlobalConfig,
}

impl PlainReader {
    pub fn new() -> Self {
        PlainReader {
            global_config: GlobalConfig::default(),
        }
    }
}

impl ConfigReader for PlainReader {
    fn read_global(&self) -> &GlobalConfig {
        &self.global_config
    }
}
