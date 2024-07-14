use std::{io::Read, thread};

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let factorio_path = args.get(1).expect("Factorio path not provided");
    let factorio_args = args.iter().skip(2).collect::<Vec<_>>();

    let mut process = std::process::Command::new(factorio_path)
        .args(factorio_args)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start Factorio");
    let output = process.stdout.as_mut().expect("Failed to read stdout");

    loop {
        let mut buffer = [0; 8024];
        let read = output
            .read(&mut buffer)
            .expect("Failed to read from stdout");
        if read == 0 {
            break;
        }

        print!("{}", String::from_utf8_lossy(&buffer[..read]));
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Message {
    SetOn { value: bool },
    SetBrightness { value: u8 },
    SetColor { r: u8, g: u8, b: u8 },
}
