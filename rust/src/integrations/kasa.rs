use crate::utils::json::boolean_int;
use crate::{
    config::CuteLightsConfig,
    utils::{future::FutureBatch, json},
};
use async_trait::async_trait;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fmt::Debug, io::Cursor};
use tokio::net::TcpStream;

use super::{Integration, Light};

// ANCHOR - KasaLight
#[derive(Debug)]
pub struct KasaLight {
    ip: String,
    is_on: bool,
    brightness: u8,
    red: u8,
    green: u8,
    blue: u8,
    supports_color: bool,
    id: String,
    name: String,
}

impl KasaLight {
    pub async fn new(ip: String) -> anyhow::Result<KasaLight> {
        let data = get_sysinfo_message();
        let response = KasaLight::send(ip.clone(), data.to_string()).await?;

        let json: serde_json::Value = serde_json::from_str(&response).unwrap();

        let state: SysInfo = serde_json::from_value(json["system"]["get_sysinfo"].clone())?;
        let (red, green, blue) = crate::utils::color::hsv_to_rgb(
            state.light_state.hue.unwrap_or(0),
            state.light_state.saturation.unwrap_or(0),
            state.light_state.brightness.unwrap_or(0),
        );

        Ok(KasaLight {
            ip,
            is_on: state.light_state.on_off,
            brightness: state.light_state.brightness.unwrap_or(0) as u8,
            red,
            green,
            blue,
            supports_color: state.is_color,
            name: state.alias,
            id: state.mic_mac,
        })
    }

    async fn send(ip: String, data: String) -> anyhow::Result<String> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let mut stream = TcpStream::connect(format!("{}:9999", ip)).await?;
        stream
            .write(
                &KasaLight::encrypt(&data)
                    .iter()
                    .map(|b| *b)
                    .collect::<Vec<u8>>(),
            )
            .await?;

        let mut buffer = Vec::new();
        loop {
            stream.read_buf(&mut buffer).await?;
            let decrypted = KasaLight::decrypt(&buffer);
            if json::is_valid(&decrypted) {
                break;
            }
        }
        Ok(KasaLight::decrypt(&buffer))
    }

    fn encrypt(input: &str) -> Vec<u8> {
        let mut key: u32 = 171;
        let mut result = Vec::new();

        // Pack the length of the string as a 4-byte unsigned integer (big-endian)
        result.write_u32::<BigEndian>(input.len() as u32).unwrap();

        for c in input.chars() {
            let a = (key ^ c as u32) as u8;
            key = a as u32;
            result.push(a);
        }

        result
    }

    fn decrypt(encrypted_bytes: &[u8]) -> String {
        let mut key: u32 = 171;
        let mut result = String::new();

        let mut cursor = Cursor::new(encrypted_bytes);
        let length = cursor.read_u32::<BigEndian>().unwrap() as usize;

        for b in &encrypted_bytes[4..] {
            let a = (key ^ (*b as u32)) as u8;
            key = *b as u32;
            result.push(a as char);
        }

        result.chars().take(length).collect()
    }
}

#[async_trait]
impl Light for KasaLight {
    async fn set_on(&mut self, on: bool) -> anyhow::Result<()> {
        let msg = on_off_message(on);
        match KasaLight::send(self.ip.clone(), msg.to_string()).await {
            Ok(_) => {
                self.is_on = on;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    async fn set_color(&mut self, red: u8, green: u8, blue: u8) -> anyhow::Result<()> {
        let (h, s, b) = crate::utils::color::rgb_to_hsv(red, green, blue);
        let msg = color_message(h, s, b);
        match KasaLight::send(self.ip.clone(), msg.to_string()).await {
            Ok(_) => {
                self.red = red;
                self.green = green;
                self.blue = blue;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    async fn set_brightness(&mut self, brightness: u8) -> anyhow::Result<()> {
        let msg = brightness_message(brightness as i64);
        match KasaLight::send(self.ip.clone(), msg.to_string()).await {
            Ok(_) => {
                self.brightness = brightness;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn id(&self) -> String {
        format!("kasa::{}", self.id)
    }

    fn red(&self) -> u8 {
        self.red
    }

    fn green(&self) -> u8 {
        self.green
    }

    fn blue(&self) -> u8 {
        self.blue
    }

    fn brightness(&self) -> u8 {
        self.brightness
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

// ANCHOR - KasaConfig

#[derive(Debug, serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct KasaConfig {
    pub enabled: bool,
    pub addresses: Vec<String>,
}

// ANCHOR - KasaIntegration

pub struct KasaIntegration;

#[async_trait]
impl Integration for KasaIntegration {
    fn name() -> String {
        "kasa".to_string()
    }

    fn preflight(config: &CuteLightsConfig) -> bool {
        config.kasa.enabled
    }
    async fn discover(config: &'static CuteLightsConfig) -> anyhow::Result<Vec<Box<dyn Light>>> {
        let mut lights = FutureBatch::new();

        for address in &config.kasa.addresses {
            let address = address.clone();
            lights.push(async move {
                match KasaLight::new(address).await {
                    Ok(light) => Some(Box::new(light) as Box<dyn Light>),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        None
                    }
                }
            });
        }

        Ok(lights.run().await.into_iter().flatten().collect())
    }
}

// ANCHOR - Messages

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct SysInfo {
    alias: String,
    mic_mac: String,
    #[serde(deserialize_with = "boolean_int")]
    is_color: bool,
    light_state: LightState,
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]

struct LightState {
    #[serde(deserialize_with = "boolean_int")]
    on_off: bool,
    brightness: Option<i64>,
    hue: Option<i64>,
    saturation: Option<i64>,
}

fn on_off_message(on: bool) -> serde_json::Value {
    json!({
        "smartlife.iot.smartbulb.lightingservice": {
            "transition_light_state": {
                "on_off": if on { 1 } else { 0 },
                "transition_period": 0
            }
        }
    })
}

fn color_message(h: i64, s: i64, b: i64) -> serde_json::Value {
    json!({
        "smartlife.iot.smartbulb.lightingservice": {
            "transition_light_state": {
                "on_off": 1,
                "hue": h,
                "saturation": s,
                "brightness": b,
                "transition_period": 0
            }
        }
    })
}

fn brightness_message(brightness: i64) -> serde_json::Value {
    json!({
        "smartlife.iot.smartbulb.lightingservice": {
            "transition_light_state": {
                "on_off": 1,
                "brightness": brightness,
                "transition_period": 0
            }
        }
    })
}

fn get_sysinfo_message() -> serde_json::Value {
    json!({
        "system": {
            "get_sysinfo": {}
        }
    })
}
