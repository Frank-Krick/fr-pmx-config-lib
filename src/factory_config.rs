use serde::{Deserialize, Serialize};
use std::fs::{write, File};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelStripConfig {
    pub cross_fader_plugin_url: String,
    pub saturator_plugin_url: String,
    pub compressor_plugin_url: String,
    pub equalizer_plugin_url: String,
    pub gain_plugin_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FactoryConfig {
    pub channel_strip: ChannelStripConfig,
}

pub fn read_factory_config() -> FactoryConfig {
    let home_dir = home::home_dir().unwrap();
    let path = home_dir.join(".config/pmx-1/pmx_factory_config.toml");
    let display = path.display();

    match File::open(&path) {
        Err(_why) => {
            let default = FactoryConfig {
                channel_strip: ChannelStripConfig {
                    saturator_plugin_url: String::from(
                        "http://calf.sourceforge.net/plugins/Saturator",
                    ),
                    compressor_plugin_url: String::from(
                        "http://calf.sourceforge.net/plugins/Compressor",
                    ),
                    equalizer_plugin_url: String::from("http://distrho.sf.net/plugins/3BandEQ"),
                    gain_plugin_url: String::from(
                        "http://kxstudio.sf.net/carla/plugins/audiogain_s",
                    ),
                    cross_fader_plugin_url: String::from("http://gareus.org/oss/lv2/xfade"),
                },
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
