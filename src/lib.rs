mod kvm_switch;
mod osd_control;
mod usb_device;

pub use kvm_switch::KvmSwitch;
pub use osd_control::OsdControl;
pub use usb_device::UsbDevice;

// mod kvm_switch;
// mod usb_device;

// use std::thread;
// use std::time::Duration;

// use rusb::{Context, Device, DeviceHandle, Direction, Recipient, RequestType, UsbContext};

// const KVM_B: u8 = 0;
// const KVM_C: u8 = 1;

// pub fn testing_thing() {
//     // Initialize the USB context
//     let context = Context::new().expect("Failed to initialize USB context");

//     // Vendor ID and Product ID of the target device
//     const VENDOR_ID: u16 = 0x2109;
//     const PRODUCT_ID: u16 = 0x8883;

//     // Find the device with the specified Vendor ID and Product ID
//     let device = find_device(&context, VENDOR_ID, PRODUCT_ID)
//         .expect("Device not found or permission denied");

//     let handle = device.open().unwrap();

//     let kvm_bc = get_osd(&handle, &vec![0xE0, 0x69]);

//     println!("curr status = {}", kvm_bc);

//     let new_kvm_bc = if kvm_bc == KVM_B.into() { KVM_C } else { KVM_B };

//     set_osd(&handle, &vec![0xE0, 0x69, new_kvm_bc]);
// }

// fn get_osd(handle: &DeviceHandle<Context>, data: &[u8]) -> u16 {
//     let buffer = [
//         vec![0x6E, 0x51, (0x81 + data.len()).try_into().unwrap(), 0x01].as_slice(),
//         data,
//     ]
//     .concat();
//     handle
//         .write_control(
//             rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device),
//             178,
//             0,
//             0,
//             &buffer,
//             Duration::from_secs(1),
//         )
//         .unwrap();
//     thread::sleep(Duration::from_millis(50));

//     let mut response: Vec<u8> = vec![0; 12];
//     handle
//         .read_control(
//             rusb::request_type(Direction::In, RequestType::Vendor, Recipient::Device),
//             162,
//             0,
//             111,
//             &mut response,
//             Duration::from_secs(1),
//         )
//         .unwrap();
//     thread::sleep(Duration::from_millis(150));

//     u16::from_le_bytes(response[9..11].try_into().unwrap())
// }

// fn set_osd(handle: &DeviceHandle<Context>, data: &[u8]) {
//     let buffer = [
//         vec![0x6E, 0x51, (0x81 + data.len()).try_into().unwrap(), 0x03].as_slice(),
//         data,
//     ]
//     .concat();

//     handle
//         .write_control(
//             rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device),
//             178,
//             0,
//             0,
//             &buffer,
//             Duration::from_secs(1),
//         )
//         .unwrap();
//     thread::sleep(Duration::from_millis(150));
// }

// // fn get_kvm_setting(handle: DeviceHandle<Context>) {
// //     let request_type = rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
// //     handle.write_control(request_type, REQUEST_OSD_SEND, 0, 0, buf, timeout);
// // }

// fn find_device(context: &Context, vendor_id: u16, product_id: u16) -> Option<Device<Context>> {
//     for device in context.devices().unwrap().iter() {
//         let device_desc = device.device_descriptor().unwrap();

//         if device_desc.vendor_id() == vendor_id && device_desc.product_id() == product_id {
//             return Some(device);
//         }
//     }

//     None
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(4, 4);
//     }
// }
