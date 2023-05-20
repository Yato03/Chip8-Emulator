
use minifb::Key;

pub fn update_keypad(keypad: &mut [bool; 16], keys: &[Key]) {

    // Inicializar todas las teclas en false
    for key in keypad.iter_mut() {
        *key = false;
    }

    for key in keys {
        match key {
            Key::Key1 => keypad[0x1] = true,
            Key::Key2 => keypad[0x2] = true,
            Key::Key3 => keypad[0x3] = true,
            Key::Key4 => keypad[0xC] = true,
            Key::Q => keypad[0x4] = true,
            Key::W => keypad[0x5] = true,
            Key::E => keypad[0x6] = true,
            Key::R => keypad[0xD] = true,
            Key::A => keypad[0x7] = true,
            Key::S => keypad[0x8] = true,
            Key::D => keypad[0x9] = true,
            Key::F => keypad[0xE] = true,
            Key::Z => keypad[0xA] = true,
            Key::X => keypad[0x0] = true,
            Key::C => keypad[0xB] = true,
            Key::V => keypad[0xF] = true,
            _ => (),
        }
        println!("Key: {:?}", key);
    }
}