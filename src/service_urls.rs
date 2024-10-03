use serde::{Deserialize, Serialize};
use std::fs::{write, File};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceUrls {
    pub pipewire_registry_url: String,
    pub pmx_registry_url: String,
    pub mod_host_addr: String,
    pub mod_host_port: u16,
    pub mod_host_feedback_port: u16,
    pub pmx_mod_host_proxy_url: String,
    pub pmx_factory_url: String,
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
                mod_host_addr: String::from("127.0.0.1"),
                mod_host_port: 5555,
                mod_host_feedback_port: 6666,
                pmx_mod_host_proxy_url: String::from("http://127.0.0.1:50031"),
                pmx_factory_url: String::from("http://127.0.0.1:50033"),
            };
            write(path, toml::to_string_pretty(&default).unwrap()).unwrap();
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
