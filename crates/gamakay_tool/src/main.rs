#![feature(generic_const_exprs)]

use anyhow::{Context, Error, Result};
// use clap::Parser;

extern crate hidapi;
use hidapi::{DeviceInfo, HidApi, HidDevice, HidError};

const VENDOR_ID: u16 = 0x3151;
const PRODUCT_ID: u16 = 0x4015;

mod gui;
mod utils;

/// CLI arguments
// #[derive(Parser, Debug)]
// #[clap(author = "Jasper Albering", version, about)]
// pub struct Args {

// }

fn main() -> Result<(), Error> {
    // let args = Args::parse();
    let keyboard_device = KeyboardDevice::new().context("HID ERROR")?;
    gui::run_gui(keyboard_device).context("GUI Error")
}

// Thanks Jan Metzger for the great article that helped me get started:
// https://zazama.de/blog/reverse-engineering-usb-rgb-devices-and-building-your-own-rgb-software-by-example-using-rust-and-glorious-model-o/

pub struct KeyboardDevice {
    control_device: HidDevice,
}

impl KeyboardDevice {
    pub fn new() -> Result<Self, HidError> {
        let api = HidApi::new()?;

        let devices = Self::filter_controllable_devices(&api);
        let control_device = Self::find_control_device(&api, &devices);

        if control_device.is_some() {
            return Ok(KeyboardDevice {
                control_device: control_device.unwrap(),
            });
        }

        return Err(HidError::HidApiError {
            message: "Device not found".to_owned(),
        });
    }

    // Filter device list for vendor / product id
    fn filter_controllable_devices(api: &HidApi) -> Vec<&DeviceInfo> {
        return api
            .device_list()
            .filter(|info| info.vendor_id() == VENDOR_ID && info.product_id() == PRODUCT_ID)
            .collect();
    }

    fn find_control_device(api: &HidApi, devices: &Vec<&DeviceInfo>) -> Option<HidDevice> {
        return devices
            .iter()
            .filter_map(|info| info.open_device(&api).ok())
            // Test if device is responsive
            // .filter(|device| {
            //     let mut buffer: [u8; 65] = [0x00; 65];
            //     return device.send_feature_report(&mut buffer).is_ok();
            // })
            .next();
    }

    pub fn send_key_remap(&self, matrix_index: u8, action_bytes: &[u8; 4]) -> Result<(), Error> {
        let firstpart: [u8; 7] = [0x13, 0x00, matrix_index, 0x00, 0x00, 0x00, 0x00];
        let checksum = calculate_checksum(&firstpart[..]);
        let padding: [u8; 52] = [0x00; 52];
        // let values = [0u8, KeyCode::G as u8, KeyCode::Fn as u8, 0u8];
        let data = [
            &[0x00 as u8][..],
            &firstpart[..],
            &[checksum][..],
            &action_bytes[..],
            &padding[..],
        ]
        .concat();
        self.control_device
            .send_feature_report(&data[..])
            .context("HID Error")
    }
}

fn calculate_checksum(data: &[u8]) -> u8 {
    let sum: u8 = data.iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
    !sum
}
