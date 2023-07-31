use gigabyte_kvm_switch::{KvmSwitch, OsdControl, UsbDevice};

const VENDOR_ID: u16 = 0x2109;
const PRODUCT_ID: u16 = 0x8883;

fn main() {
    let device = match UsbDevice::open(VENDOR_ID, PRODUCT_ID) {
        Some(device) => device,
        None => {
            eprintln!(
                "Could not open device with vendor id {} and product id {}",
                VENDOR_ID, PRODUCT_ID
            );
            return;
        }
    };
    let osd_control = OsdControl::new(device);
    let kvm_switch = KvmSwitch::new(osd_control);

    println!("Toggling KVM...");
    kvm_switch.toggle_kvm();
}
