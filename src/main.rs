mod chip8;  
use chip8::Chip8;

mod display;
use display::Display;

mod input;

use std::time::Instant;
use std::time::Duration;
use std::thread;
use minifb::{Window, Key};

fn main() {
    let rom_name = "Pong";
    let mut chip8 = Chip8::new();
    let mut display = Display::new(rom_name);

    let rom_path = format!("C:\\Users\\migue\\Documents\\Programación\\rust\\chip8_emulator\\assets\\roms\\{}.ch8", rom_name);
    /* Cargar el contenido de la ROM desde un archivo o cualquier otra fuente */

    let result = chip8.load_rom(&rom_path);
    println!("Result: {:?}", result);
    
    while should_continue(&display.get_window()) {
        if chip8.get_pc() < 0xfff {
            let start_frame = Instant::now();
            for _ in 0..700/60 {
                chip8.emulate_cycle();
            }
            let end_frame = Instant::now();

            let elapsed = end_frame.duration_since(start_frame);

            let elapsed_ms = elapsed.as_millis();
            let time_to_sleep = if 17 > elapsed_ms {17 - elapsed_ms}  else {0};
            thread::sleep(Duration::from_millis(time_to_sleep as u64));

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
