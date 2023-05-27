mod input;
mod display;
use display::Display;

#[cfg(test)]
mod tests {
    use minifb::Key;
    use super::input::update_keypad;

    #[test]
    fn update_keypad_test() {
        let mut keypad = [false; 16];
        let keys = [Key::Key1, Key::Q, Key::V];
        
        update_keypad(&mut keypad, &keys);
        print!("Keypad: {:?}\n", keypad);
        assert_eq!(keypad[0x1], true);
        assert_eq!(keypad[0x4], true);
        assert_eq!(keypad[0xF], true);
        
        // Resto de las aserciones para las teclas que no se presionaron
        assert_eq!(keypad[0x0], false);
        assert_eq!(keypad[0x2], false);
        assert_eq!(keypad[0x3], false);
        assert_eq!(keypad[0x5], false);
        assert_eq!(keypad[0x6], false);
        assert_eq!(keypad[0x7], false);
        assert_eq!(keypad[0x8], false);
        assert_eq!(keypad[0x9], false);
        assert_eq!(keypad[0xA], false);
        assert_eq!(keypad[0xB], false);
        assert_eq!(keypad[0xC], false);
        assert_eq!(keypad[0xD], false);
        assert_eq!(keypad[0xE], false);
        // Resto de las teclas...
    }

    
}
