use serde::{Deserialize, Serialize};
use std::fs::{write, File};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataFiles {
    pub pmx_registry_data_file: String,
    pub pmx_registry_output_data_file: String,
}

pub fn read_data_file_paths() -> DataFiles {
    let home_dir = home::home_dir().unwrap();
    let path = home_dir.join(".config/pmx-1/data_files.toml");
    let display = path.display();

    match File::open(&path) {
        Err(_why) => {
            let default = DataFiles {
                pmx_registry_data_file: String::from(
                    home_dir
                        .join(".config/pmx-1/pmx_registry.json")
                        .to_str()
                        .unwrap(),
                ),
                pmx_registry_output_data_file: String::from(
                    home_dir
                        .join(".config/pmx-1/pmx_registry_outputs.json")
                        .to_str()
                        .unwrap(),
                ),
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
