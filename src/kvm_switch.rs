use crate::osd_control::OsdControl;

const KVM_B: u8 = 0;
const KVM_C: u8 = 1;

pub struct KvmSwitch {
    osd_control: OsdControl,
}

impl KvmSwitch {
    pub fn new(osd_control: OsdControl) -> Self {
        KvmSwitch { osd_control }
    }

    pub fn toggle_kvm(&self) {
        let kvm_bc = self.osd_control.get_osd(&vec![0xE0, 0x69]);
        let new_kvm_bc = match kvm_bc as u8 {
            KVM_B => KVM_C,
            KVM_C => KVM_B,
            _ => panic!("Unexpected KVM state {}", kvm_bc),
        };

        match new_kvm_bc {
            KVM_B => println!("Switching KVM to USB B"),
            KVM_C => println!("Switching KVM to USB C"),
            _ => panic!("Unexpected KVM state {}", new_kvm_bc),
        };

        self.osd_control.set_osd(&vec![0xE0, 0x69, new_kvm_bc]);
    }
}
