use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut device = VirtualDevice::default().unwrap();

    thread::sleep(Duration::from_secs(2));

    // type hello
    for key in [KEY_H, KEY_E, KEY_L, KEY_L, KEY_O, KEY_SPACE, KEY_W, KEY_O, KEY_R, KEY_L, KEY_D] {
        device.click(key).unwrap();
    }
}
