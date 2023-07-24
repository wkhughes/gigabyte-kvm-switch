use rusb::{Context, Device, DeviceHandle, UsbContext};

pub struct UsbDevice {
    pub handle: DeviceHandle<Context>,
}

impl UsbDevice {
    // Open the USB device identified by vendor ID and product ID
    pub fn open(vendor_id: u16, product_id: u16) -> Option<Self> {
        let context = Context::new().expect("Failed to initialize USB context");

        let device = find_device(&context, vendor_id, product_id)?;
        let handle = device.open().expect("Failed to open USB device");
        Some(Self { handle })
    }
}

fn find_device(context: &Context, vendor_id: u16, product_id: u16) -> Option<Device<Context>> {
    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if device_desc.vendor_id() == vendor_id && device_desc.product_id() == product_id {
            return Some(device);
        }
    }

    None
}
