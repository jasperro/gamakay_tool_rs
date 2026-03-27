#![feature(generic_const_exprs)]

use anyhow::{Context, Error, Result};
// use clap::Parser;

extern crate hidapi;
use hidapi::{DeviceInfo, HidApi, HidDevice, HidError};

use crate::utils::{KEYBOARD_LAYOUT, KeyAction, KeyCode, KeyboardLayoutExt};

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

    pub fn send_key_remap(&self) -> Result<(), Error> {
        let keytomodify: u8 = KEYBOARD_LAYOUT
            .find_by_key(KeyCode::B)
            .context("Key does not exist in layout")?
            .matrix_index;
        let firstpart: [u8; 7] = [0x13, 0x00, keytomodify, 0x00, 0x00, 0x00, 0x00];
        let checksum = calculate_checksum(&firstpart[..]);
        let padding: [u8; 52] = [0x00; 52];
        // let values = [0u8, KeyCode::G as u8, KeyCode::Fn as u8, 0u8];
        let values = KeyCode::B;
        let values = values.to_bytes();
        let data = [
            &[0x00 as u8][..],
            &firstpart[..],
            &[checksum][..],
            &values[..],
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
