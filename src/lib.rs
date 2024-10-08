use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

mod builder_config;
mod data_file_paths;
mod factory_config;
mod service_urls;

pub use crate::data_file_paths::{read_data_file_paths, DataFiles};
pub use crate::factory_config::{read_factory_config, ChannelStripConfig, FactoryConfig};
pub use crate::service_urls::{read_service_urls, ServiceUrls};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub targets: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MixerConfig {
    pub main_output: Output,
    pub headphones_output: Output,
}

pub fn read_mixer_configuration_file() -> MixerConfig {
    let home_dir = home::home_dir().unwrap();
    let path = home_dir.join(".config/pmx-1/config.toml");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut raw_string = String::new();

    if let Err(why) = file.read_to_string(&mut raw_string) {
        panic!("couldn't read {}: {}", display, why);
    }

    toml::from_str(raw_string.as_str()).unwrap()
}
