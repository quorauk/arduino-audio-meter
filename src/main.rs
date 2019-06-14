extern crate serialport;
extern crate winapi;

use std::io::{self, Write};

use serialport::*;
use std::time::Duration;

mod windows;

const NUM_PINS : f32 = 10 as f32;
const PORT_NAME: &str = "COM3";

fn get_settings() -> SerialPortSettings {
    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(100);
    settings.baud_rate = 9600;
    return settings
}

fn connected(port : &mut serialport::SerialPort, input_device : *mut winapi::um::endpointvolume::IAudioMeterInformation) {
    let mut x : u32;
    loop {
        let mut peak : f32 = 0.0;
        unsafe { (*input_device).GetPeakValue(&mut peak) };

        x = (peak * NUM_PINS) as u32;
        let s : String = x.to_string() + "\n";  
        match port.write(s.as_bytes()) {
            Ok(_) => {
                std::io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                eprint!("Timeout: {:?}", e);
                return;
            }
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}


fn main() {

    let input_device = windows::get_audio_meter_information();
    let settings = get_settings();

    loop {
        match serialport::open_with_settings(&PORT_NAME, &settings) {
            Ok(mut port) => {
                println!("Opened serial device: {}", PORT_NAME);
                connected(&mut *port, input_device);
            }
            Err(e) => {
                eprintln!("Failed to open port: {}", e);
                std::thread::sleep(Duration::from_secs(5));
            }
        }
    }
}