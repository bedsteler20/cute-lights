use serde::{Deserialize, Serialize};

use crate::integrations::{govee::GoveeConfig, hue::HueConfig, kasa::KasaConfig};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct CuteLightsConfig {
    pub kasa: KasaConfig,
    pub govee: GoveeConfig,
    pub hue: HueConfig,
}

impl CuteLightsConfig {
    pub fn load_default() -> CuteLightsConfig {
        if let Ok(config) = std::env::var("CUTE_LIGHTS_CONFIG_PATH") {
            CuteLightsConfig::load_from_file(&config)
        } else {
            let cfg_home = std::env::var("XDG_CONFIG_HOME")
                .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap()));
            let config = format!("{}/cute_lights/lights.toml", cfg_home);
            if std::path::Path::new(&config).exists() {
                CuteLightsConfig::load_from_file(&config)
            } else {
                CuteLightsConfig::default()
            }
        }
    }

    pub fn load_from_file(file: &str) -> CuteLightsConfig {
        let config = std::fs::read_to_string(file).unwrap();
        toml::from_str(&config).unwrap()
    }
}

