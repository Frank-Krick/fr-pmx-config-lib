use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceUrls {
    pub pipewire_registry_url: String,
    pub pmx_registry_url: String,
}

pub fn read_service_urls() -> ServiceUrls {
    let home_dir = home::home_dir().unwrap();
    let path = home_dir.join(".config/pmx-1/service_urls.toml");
    let display = path.display();

    match File::open(&path) {
        Err(_why) => {
            let default = ServiceUrls {
                pipewire_registry_url: String::from("http://127.0.0.1:50000"),
                pmx_registry_url: String::from("http://127.0.0.1:50001"),
            };
            fs::write(path, toml::to_string_pretty(&default).unwrap()).unwrap();
            default
        }
        Ok(mut file) => {
            let mut raw_string = String::new();

            if let Err(why) = file.read_to_string(&mut raw_string) {
                panic!("couldn't read {}: {}", display, why);
            }

            toml::from_str(raw_string.as_str()).unwrap()
        }
    }
}
