mod chip8;  

use chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    let rom_path = "C:\\Users\\migue\\Documents\\Programación\\rust\\chip8_emulator\\assets\\roms\\Pong.ch8";
    /* Cargar el contenido de la ROM desde un archivo o cualquier otra fuente */

    let result = chip8.load_rom(&rom_path);
    println!("Result: {:?}", result);
    loop {
        if chip8.get_pc() < 0xfff {
            chip8.emulate_cycle();
        }
    }
}
