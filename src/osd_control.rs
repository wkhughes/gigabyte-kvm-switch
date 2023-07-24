use rusb::{Direction, Recipient, RequestType};
use std::thread;
use std::time::Duration;

use crate::usb_device::UsbDevice;

pub struct OsdControl {
    device: UsbDevice,
}

impl OsdControl {
    pub fn new(device: UsbDevice) -> Self {
        OsdControl { device }
    }

    pub fn get_osd(&self, data: &[u8]) -> u16 {
        let buffer = [
            vec![0x6E, 0x51, (0x81 + data.len()) as u8, 0x01].as_slice(),
            data,
        ]
        .concat();
        self.device
            .handle
            .write_control(
                rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device),
                178,
                0,
                0,
                &buffer,
                Duration::from_secs(1),
            )
            .unwrap();

        // Replicate sleep of the official OSD sidekick
        thread::sleep(Duration::from_millis(50));

        let mut response: Vec<u8> = vec![0; 12];
        self.device
            .handle
            .read_control(
                rusb::request_type(Direction::In, RequestType::Vendor, Recipient::Device),
                162,
                0,
                111,
                &mut response,
                Duration::from_secs(1),
            )
            .unwrap();

        // Replicate sleep of the official OSD sidekick
        thread::sleep(Duration::from_millis(150));

        u16::from_le_bytes(response[9..11].try_into().unwrap())
    }

    pub fn set_osd(&self, data: &[u8]) {
        let buffer = [
            vec![0x6E, 0x51, (0x81 + data.len()) as u8, 0x03].as_slice(),
            data,
        ]
        .concat();
        self.device
            .handle
            .write_control(
                rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device),
                178,
                0,
                0,
                &buffer,
                Duration::from_secs(1),
            )
            .unwrap();

        // Replicate sleep of the official OSD sidekick
        thread::sleep(Duration::from_millis(150));
    }
}
