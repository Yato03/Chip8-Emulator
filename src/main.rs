mod chip8;  
use chip8::Chip8;

mod display;
use display::Display;

mod input;

use minifb::{Window, Key};

fn main() {
    let mut chip8 = Chip8::new();
    let mut display = Display::new();

    let rom_path = "C:\\Users\\migue\\Documents\\Programación\\rust\\chip8_emulator\\assets\\roms\\Pong.ch8";
    /* Cargar el contenido de la ROM desde un archivo o cualquier otra fuente */

    let result = chip8.load_rom(&rom_path);
    println!("Result: {:?}", result);
    
    while should_continue(&display.get_window()) {
        if chip8.get_pc() < 0xfff {
            chip8.emulate_cycle();
            display.update(chip8.get_screen());

            let keys = display.get_window().get_keys().unwrap_or(Vec::new());
            input::update_keypad(&mut chip8.keypad, &keys);
        }
    }
    std::process::exit(0);
}

fn should_continue(window: &Window) -> bool {
    window.is_open() && !window.is_key_down(Key::Escape)
}
