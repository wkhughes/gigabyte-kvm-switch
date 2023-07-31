# Gigabyte KVM Switch

Toggles the KVM setting of a Gigabyte monitor. Only tested with Gigabyte M27Q monitor.

The commands were reverse engineered from the official Gigabyte OSD Sidekick.

## Usage

Run the executable! Tested on Windows and Mac.

## Troubleshooting

### Unable to open device on Windows

If the official Gigabyte OSD Sidekick is installed on Windows, it will register the monitor USB device as a libusb-win32 device. This will likely lead to an error opening the device when running this program, as libusb cannot natively interface with this driver.

There are two solutions:

1. **Recommended: Install [libusbK](https://github.com/mcuee/libusbk/releases/tag/V3.1.0.0) with the 'Upgrade libusb-win32 components' setting**. This adds a shim so that libusb can interact with the libusb-win32 driver.
1. **Alternative: Remove libusb-win32 as the monitor USB device driver from device manager.** This will return the device driver to the default WinUSB, which libusb can interface with. However the official OSD Sidekick will no longer work.

For more details on this limitation please see the [libusb documentation](https://github.com/libusb/libusb/wiki/Windows#driver-installation).

## Building

```
cargo build --release
```
