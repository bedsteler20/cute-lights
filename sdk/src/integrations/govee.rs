use super::Integration;
use crate::utils::json::boolean_int;
use crate::{config::CuteLightsConfig, utils::future::FutureBatch, CuteResult};
use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};
use tokio::net::UdpSocket;

use super::Light;

// ANCHOR - GoveeLight
pub struct GoveeLight {
    udp_socket: Arc<UdpSocket>,
    device_addr: SocketAddr,
    is_on: bool,
    brightness: u8,
    red: u8,
    green: u8,
    blue: u8,
    id: String,
}

impl GoveeLight {
    pub async fn new(udp_socket: Arc<UdpSocket>, ip: &str, mac: &str) -> CuteResult<GoveeLight> {
        let device_addr = SocketAddr::new(IpAddr::V4(ip.parse()?), 4003);

        let msg = Request::DevStatus {};

        let response = send_message(&udp_socket, &device_addr, msg, true).await?;

        let response = match response {
            Response::DevStatus(status) => status,
            _ => return Err("Invalid response".into()),
        };

        Ok(GoveeLight {
            udp_socket,
            device_addr,
            is_on: response.on,
            brightness: response.brightness as u8,
            red: response.color.r,
            green: response.color.g,
            blue: response.color.b,
            id: mac.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl Light for GoveeLight {
    async fn set_on(&mut self, on: bool) -> CuteResult<()> {
        let msg = Request::Turn { value: on as u8 };
        send_message(&self.udp_socket, &self.device_addr, msg, false).await?;
        self.is_on = on;
        Ok(())
    }

    async fn set_color(&mut self, red: u8, green: u8, blue: u8) -> CuteResult<()> {
        let msg = Request::Color {
            color: DeviceColor {
                r: red,
                g: green,
                b: blue,
            },
            color_temperature_kelvin: 7200,
        };
        send_message(&self.udp_socket, &self.device_addr, msg, false).await?;
        self.red = red;
        self.green = green;
        self.blue = blue;
        Ok(())
    }

    async fn set_brightness(&mut self, brightness: u8) -> CuteResult<()> {
        let msg = Request::Brightness {
            value: brightness as u8,
        };
        send_message(&self.udp_socket, &self.device_addr, msg, false).await?;
        self.brightness = brightness;
        Ok(())
    }

    fn id(&self) -> String {
        format!("govee::{}", self.id)
    }

    fn is_on(&self) -> bool {
        self.is_on
    }
    fn name(&self) -> String {
        format!("Govee Light ({})", self.id)
    }
    fn supports_color(&self) -> bool {
        true
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
}

// ANCHOR - GoveeConfig

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct GoveeConfig {
    pub enabled: bool,
    pub addresses: Vec<String>,
    #[serde(default = "default_scan_timeout")]
    pub scan_timeout: u64,
}

fn default_scan_timeout() -> u64 {
    5000
}

// ANCHOR - GoveeIntegration

pub struct GoveeIntegration;

#[async_trait::async_trait]
impl Integration for GoveeIntegration {
    fn name() -> String {
        "govee".to_string()
    }
    async fn discover(config: &'static CuteLightsConfig) -> CuteResult<Vec<Box<dyn Light>>> {
        let mut batch = FutureBatch::new();
        let client_sock = Arc::new(UdpSocket::bind("0.0.0.0:4002").await?);

        let discovered = tokio::time::timeout(
            std::time::Duration::from_millis(config.integrations.govee.scan_timeout),
            discover_ids(
                client_sock.clone(),
                config.integrations.govee.addresses.clone(),
            ),
        )
        .await??;

        for (ip, mac) in discovered {
            let client_sock = client_sock.clone();
            batch.push(async move {
                match GoveeLight::new(client_sock, &ip, &mac).await {
                    Ok(light) => Some(Box::new(light) as Box<dyn Light>),
                    Err(e) => {
                        eprintln!(
                            "Failed to connect to Govee light at {} {:?}: {}",
                            ip, mac, e
                        );
                        None
                    }
                }
            });
        }

        Ok(batch.run().await.into_iter().flatten().collect())
    }

    fn preflight(config: &CuteLightsConfig) -> bool {
        config.integrations.govee.enabled
    }
}

// ANCHOR - Multicast Discovery

const MULTICAST_GROUP: &str = "239.255.255.250";
const MULTICAST_PORT: u16 = 4001;
const MULTICAST_TTL: u32 = 2;

pub async fn discover_ids(
    client_sock: Arc<UdpSocket>,
    ips: Vec<String>,
) -> CuteResult<Vec<(String, String)>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async move {
        client_sock
            .join_multicast_v4(
                Ipv4Addr::from_str(MULTICAST_GROUP).unwrap(),
                Ipv4Addr::UNSPECIFIED,
            )
            .expect("Failed to join multicast group");

        let mut buf = [0; 10240];
        loop {
            if let Ok((size, _)) = client_sock.recv_from(&mut buf).await {
                let res = tx
                    .send(String::from_utf8_lossy(&buf[..size]).to_string())
                    .await;
                if let Err(_) = res {
                    break;
                }
            }
        }

        client_sock
            .leave_multicast_v4(
                Ipv4Addr::from_str(MULTICAST_GROUP).unwrap(),
                Ipv4Addr::UNSPECIFIED,
            )
            .expect("Failed to leave multicast group");
    });

    tokio::spawn(async move {
        let message = r#"
        {
            "msg": {
                "cmd": "scan",
                "data": {
                    "account_topic": "reserve"
                }
            }
        }
    "#;

        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .expect("Failed to bind socket");
        socket
            .set_multicast_ttl_v4(MULTICAST_TTL)
            .expect("Failed to set TTL");

        let json_result = message.trim().to_string();

        socket
            .send_to(
                json_result.as_bytes(),
                format!("{}:{}", MULTICAST_GROUP, MULTICAST_PORT),
            )
            .await
            .expect("Failed to send message");
    });

    let mut results = Vec::new();

    while let Some(message) = rx.recv().await {
        let response: ResponseMessage = serde_json::from_str(&message).unwrap();

        if let Response::Scan(device) = response.msg {
            if ips.contains(&device.ip.to_string()) {
                results.push((device.ip.to_string(), device.device));
            }
        }

        if results.len() == ips.len() {
            break;
        }
    }

    rx.close();

    Ok(results)
}
// ANCHOR - Messages

async fn send_message(
    sock: &UdpSocket,
    addr: &SocketAddr,
    data: Request,
    expect_response: bool,
) -> CuteResult<Response> {
    sock.send_to(
        serde_json::to_string(&RequestMessage { msg: data })?.as_bytes(),
        addr,
    )
    .await?;

    if !expect_response {
        return Ok(Response::Void);
    }
    let mut buf = [0; 1024];

    let (amt, _) = sock.recv_from(&mut buf).await?;

    let response: ResponseMessage = serde_json::from_str(&String::from_utf8_lossy(&buf[..amt]))?;

    Ok(response.msg)
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AccountTopic {
    #[serde(rename = "reserve")]
    Reserve,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "cmd", content = "data")]
pub enum Request {
    #[serde(rename = "scan")]
    Scan { topic: AccountTopic },
    #[serde(rename = "devStatus")]
    DevStatus {},
    #[serde(rename = "turn")]
    Turn { value: u8 },
    #[serde(rename = "brightness")]
    Brightness { value: u8 },
    #[serde(rename = "colorwc")]
    Color {
        color: DeviceColor,
        #[serde(rename = "colorTemInKelvin")]
        color_temperature_kelvin: u32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cmd", content = "data")]
pub enum Response {
    #[serde(rename = "scan")]
    Scan(LanDevice),
    #[serde(rename = "devStatus")]
    DevStatus(DeviceStatus),
    Void,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeviceStatus {
    #[serde(rename = "onOff", deserialize_with = "boolean_int")]
    pub on: bool,
    pub brightness: u8,
    pub color: DeviceColor,
    #[serde(rename = "colorTemInKelvin")]
    pub color_temperature_kelvin: u32,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct DeviceColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanDevice {
    pub ip: IpAddr,
    pub device: String,
    pub sku: String,
    #[serde(rename = "bleVersionHard")]
    pub ble_version_hard: String,
    #[serde(rename = "bleVersionSoft")]
    pub ble_version_soft: String,
    #[serde(rename = "wifiVersionHard")]
    pub wifi_version_hard: String,
    #[serde(rename = "wifiVersionSoft")]
    pub wifi_version_soft: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMessage {
    msg: Request,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    msg: Response,
}
