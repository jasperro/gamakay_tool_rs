use anyhow::{Context, Error, Result};
// use clap::Parser;

extern crate hidapi;
use hidapi::{DeviceInfo, HidApi, HidDevice, HidError};

const VENDOR_ID: u16 = 0x3151;
const PRODUCT_ID: u16 = 0x4015;

mod utils;

/// CLI arguments
// #[derive(Parser, Debug)]
// #[clap(author = "Jasper Albering", version, about)]
// pub struct Args {

// }

fn main() -> Result<(), Error> {
    // let args = Args::parse();
    let keyboard_device = KeyboardDevice::new().context("HID ERROR")?;
    keyboard_device.send_key_remap()?;
    anyhow::Ok(())
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

    pub fn send_key_remap(&self) -> Result<(), HidError> {
        let keytomodify: u8 = 38;
        // Any of the 4 values can set the key.
        // Best practice is probably modifiers first, then other key.
        let value1: u8 = 0;
        let value2: u8 = 28;
        let value3: u8 = 0;
        let value4: u8 = 0;
        let firstpart: [u8; 7] = [0x13, 0x00, keytomodify, 0x00, 0x00, 0x00, 0x00];
        let checksum = calculate_checksum(&firstpart[..]);
        let secondpart: [u8; 57] = [
            checksum, value1, value2, value3, value4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
        ];
        let data = [&[0x00 as u8][..], &firstpart[..], &secondpart[..]].concat();
        self.control_device.send_feature_report(&data[..])
    }
}

fn calculate_checksum(data: &[u8]) -> u8 {
    let sum: u8 = data.iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
    !sum
}
