use crate::{utils::json, CuteResult};

use super::Light;

// ANCHOR - HueLight

#[derive(Debug)]
pub struct HueLight {
    id: String,
    username: String,
    bridge: String,
    saturation: i64,
    hue: i64,
    brightness: i64,
    name: String,
    supports_color: bool,
    is_on: bool,
}

#[async_trait::async_trait]
impl Light for HueLight {
    async fn set_on(&mut self, on: bool) -> CuteResult<()> {
        let url = format!(
            "http://{}/api/{}/lights/{}/state",
            self.bridge, self.username, self.id
        );
        let body = serde_json::json!({"on": on});
        let client = reqwest::Client::new();
        client.put(&url).body(body.to_string()).send().await?;
        self.is_on = on;
        Ok(())
    }

    async fn set_brightness(&mut self, brightness: i64) -> CuteResult<()> {
        let url = format!(
            "http://{}/api/{}/lights/{}/state",
            self.bridge, self.username, self.id
        );
        let body = serde_json::json!({"bri": (brightness as f64 / 100.0 * 254.0).round() as i64});
        let client = reqwest::Client::new();
        client.put(&url).body(body.to_string()).send().await?;
        self.brightness = brightness;
        Ok(())
    }

    async fn set_color(&mut self, hue: i64, saturation: i64, brightness: i64) -> CuteResult<()> {
        let url = format!(
            "http://{}/api/{}/lights/{}/state",
            self.bridge, self.username, self.id
        );
        let body = serde_json::json!({
            "hue": (hue as f64 / 360.0 * 65535.0).round() as i64,
            "sat": (saturation as f64 / 100.0 * 254.0).round() as i64,
            "bri": (brightness as f64 / 100.0 * 254.0).round() as i64
        });
        let client = reqwest::Client::new();
        client.put(&url).body(body.to_string()).send().await?;

        self.hue = hue;
        self.saturation = saturation;
        self.brightness = brightness;
        Ok(())
    }
    
    fn id(&self) -> String {
        format!("hue::{}", self.id)
    }
    
    fn brightness(&self) -> i64 {
        self.brightness
    }

    fn hue(&self) -> i64 {
        self.hue
    }

    fn saturation(&self) -> i64 {
        self.saturation
    }

    fn is_on(&self) -> bool {
        self.is_on
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn supports_color(&self) -> bool {
        self.supports_color
    }
}

// ANCHOR - HueConfig
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct HueConfig {
    pub enabled: bool,
    pub bridge_ip: Option<String>,
    pub username: Option<String>,
}

// ANCHOR - HueIntegration

pub struct HueIntegration;

#[async_trait::async_trait]
impl super::Integration for HueIntegration {
    fn name() -> String {
        "hue".to_string()
    }
    async fn discover(
        config: &'static crate::config::CuteLightsConfig,
    ) -> CuteResult<Vec<Box<dyn Light>>> {
        let bridge = &config.integrations.hue.bridge_ip.as_ref().unwrap();
        let user = &config.integrations.hue.username.as_ref().unwrap();

        let mut lights = vec![];
        let url = format!("http://{}/api/{}/lights/", bridge, user);
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        let js: serde_json::Value = serde_json::from_str(&body)?;

        for (light_id, value) in json::object(&js)? {
            let is_reachable = json::bool(&value["state"]["reachable"])?;
            if !is_reachable {
                continue;
            }
            let is_on = json::bool(&value["state"]["on"])?;
            let saturation = (json::float(&value["state"]["sat"])? / 254.0 * 100.0).round() as i64;
            let hue = (json::float(&value["state"]["hue"])? / 65535.0 * 360.0).round() as i64;
            let brightness = (json::float(&value["state"]["bri"])? / 254.0 * 100.0).round() as i64;
            let name = json::object(&value)?["name"].as_str().unwrap();
            let supports_color = !&value["capabilities"]["control"]["colorgamut"].is_null();

            lights.push(HueLight {
                id: light_id.to_string(),
                username: user.to_string(),
                bridge: bridge.to_string(),
                saturation,
                hue,
                brightness,
                name: name.to_string(),
                supports_color,
                is_on,
            });
        }

        Ok(lights
            .into_iter()
            .map(|l| Box::new(l) as Box<dyn Light>)
            .collect())
    }

    fn preflight(config: &crate::config::CuteLightsConfig) -> bool {
        if !config.integrations.hue.enabled {
            return false;
        }

        if config.integrations.hue.bridge_ip.is_none() {
            eprintln!("Hue bridge not configured");
            return false;
        }

        if config.integrations.hue.username.is_none() {
            eprintln!("Hue user not configured");
            return false;
        }

        true
    }
}
