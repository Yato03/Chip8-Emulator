use std::fs;
use std::io;



pub struct Chip8 {
    // Aquí puedes definir los registros, memoria y otros componentes necesarios del emulador

    // Registros
    v: [u8; 16],     // Registros generales V0-VF
    i: u16,           // Registro de índice
    pc: u16,          // Contador de programa

    // Memoria
    memory: [u8; 4096],

    // Pila
    stack: [u16; 16],
    sp: u8,           // Puntero de pila

    // Timers
    delay_timer: u8,
    sound_timer: u8,

    // Teclado
    pub keypad: [bool; 16],

    // Pantalla
    screen: [bool; 64 * 32],

    //Fuentes
    DEFAULT_FONTS: [[u8; 5]; 16]
}

impl Chip8 {
    pub fn new() -> Self {
        // Aquí puedes inicializar y configurar el estado inicial del emulador
        // Puedes asignar valores iniciales a los registros y la memoria, por ejemplo
        // También puedes realizar otras tareas de inicialización aquí

        

        Self {
            v: [0; 16],
            i: 0,
            pc: 0x200,  // La ejecución de las ROMs comienza en la dirección 0x200
            memory: [0; 4096],
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            screen: [false; 64 * 32],
            DEFAULT_FONTS: [
                [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
                [0x20, 0x60, 0x20, 0x20, 0x70], // 1
                [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
                [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
                [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
                [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
                [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
                [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
                [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
                [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
                [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
                [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
                [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
                [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
                [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
                [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
            ]
        }
}

        
    pub fn load_rom(&mut self, rom_path: &str) -> Result<(), io::Error>  {
        // Aquí puedes implementar la lógica para cargar una ROM en la memoria del emulador
        // Puedes abrir el archivo ROM, leer su contenido y almacenarlo en la memoria adecuada
        // ...

        //Cargar fonts
        // Cargar los fuentes por defecto en la memoria de Chip-8

        const FONT_START_ADDRESS: u8 = 0x0;
        const FONT_SIZE: u8= 5;

        for i in 0..16 {
            for j in 0..FONT_SIZE {
                let index = FONT_START_ADDRESS + i * FONT_SIZE + j;
                self.memory[index as usize] = self.DEFAULT_FONTS[i as usize][j as usize];
            }
        }

        let rom = fs::read(rom_path)?;

        for (i, &byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }

        Ok(())
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_screen(&self) -> &[bool; 64 * 32] {
        &self.screen
    }

    pub fn emulate_cycle(&mut self) {
        // Aquí puedes implementar el bucle de emulación principal del CHIP-8
        // Este bucle se encargará de buscar y ejecutar las instrucciones una tras otra
        // Puedes utilizar el contador de programa (PC) para obtener las instrucciones y actualizarlo adecuadamente
        // También necesitarás decodificar las instrucciones y ejecutar las operaciones correspondientes

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
           self.sound_timer -= 1;
           //sonar sonido
        }


        // Obtener la instrucción desde la memoria en la dirección del contador de programa (PC)
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16);


        // Decodificar la instrucción y ejecutar la operación correspondiente
        Chip8::decode_and_execute_instruction(self, opcode);

        // Actualizar el contador de programa (PC) para apuntar a la siguiente instrucción
        self.pc += 2;
    }

    fn decode_and_execute_instruction(&mut self,opcode: u16) {
        let instruction = opcode & 0xF000;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let n = opcode & 0x000F;
        let nn = opcode & 0x00FF;
        let nnn = opcode & 0x0FFF;
    
        match instruction {
            0x0000 => {
                match opcode {
                    0x00E0 => {
                        // Instrucción CLS: Borrar la pantalla
                        self.screen = [false; 64 * 32];
                    },
                    0x00EE => {
                        // Instrucción RET: Retornar de una subrutina
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize] - 2;
                    },
                    _ => {
                        // Instrucción SYS: Salto a una dirección de memoria
                    }
                }
            },
            0x1000 => {
                // Instrucción JP: Salto a una dirección de memoria
                self.pc = nnn - 2;
            },
            0x2000 => {
                // Instrucción CALL: Llamar a una subrutinq
                self.stack[self.sp as usize] = self.pc+2;
                self.sp += 1;
                self.pc = nnn-2;
            },
            0x3000 => {
                // Instrucción SE: Saltar si igual
                if self.v[x as usize] == nn as u8 {
                    self.pc += 2;
                }
            },
            0x4000 => {
                // Instrucción SNE: Saltar si no es igual
                if self.v[x as usize] != nn as u8 {
                    self.pc += 2;
                }
            },
            0x5000 => {
                match n {
                    0x0 => {
                        // Instrucción SE: Saltar si igual
                        if self.v[x as usize] == self.v[y as usize] {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        // Instrucción no reconocida
                    }
                }
            },
            0x6000 => {
                // Instrucción LD: Asignar
                self.v[x as usize] = nn as u8;
            },
            0x7000 => {
                // Instrucción ADD: Sumar
                let (result, _) = self.v[x as usize].overflowing_add(nn as u8);
                self.v[x as usize] = result;
            },
            0x8000 =>{
                match n {
                    0x0 => {
                        // Instrucción LD: Asignar
                        self.v[x as usize] = self.v[y as usize];
                    },
                    0x1 => {
                        // Instrucción OR: Operación OR
                        self.v[x as usize] |= self.v[y as usize];
                    },
                    0x2 => {
                        // Instrucción AND: Operación AND
                        self.v[x as usize] &= self.v[y as usize];
                    },
                    0x3 => {
                        // Instrucción XOR: Operación XOR
                        self.v[x as usize] ^= self.v[y as usize];
                    },
                    0x4 => {
                        // Instrucción ADD: Sumar
                        let (result, overflow) = self.v[x as usize].overflowing_add(self.v[y as usize]);
                        self.v[x as usize] = result;
                        self.v[0xF] = overflow as u8;
                    },
                    0x5 => {
                        // Instrucción SUB: Restar
                        let (result, overflow) = self.v[x as usize].overflowing_sub(self.v[y as usize]);
                        self.v[x as usize] = result;
                        self.v[0xF] = !overflow as u8;
                    },
                    0x6 => {
                        // Instrucción SHR: Desplazamiento a la derecha
                        self.v[0xF] = self.v[y as usize] & 0x1;
                        self.v[x as usize] = self.v[y as usize] >> 1;
                    },
                    0x7 => {
                        // Instrucción SUBN: Restar
                        let (result, overflow) = self.v[y as usize].overflowing_sub(self.v[x as usize]);
                        self.v[x as usize] = result;
                        self.v[0xF] = !overflow as u8;
                    },
                    0xE => {
                        // Instrucción SHL: Desplazamiento a la izquierda
                        self.v[0xF] = self.v[x as usize] >> 7;
                        self.v[x as usize] = self.v[y as usize] << 1;
                    },
                    _ => {
                        // Instrucción no reconocida
                    }
                }
            },
            0x9000 => {
                match n {
                    0x0 => {
                        // Instrucción SNE: Saltar si igual
                        if self.v[x as usize] != self.v[y as usize] {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        // Instrucción no reconocida
                    }
                }
            },
            0xA000 => {
                // Instrucción LD: Asignar
                self.i = nnn;
            },
            0xB000 => {
                // Instrucción JP: Salto a una dirección de memoria
                self.pc = nnn + self.v[0] as u16 - 2;
            },
            0xC000 => {
                // Instrucción RND: Generar número aleatorio
                self.v[x as usize] = rand::random::<u8>() & nn as u8;
            },
            0xD000 =>{
                // Instrucción DRW: Dibujar sprite
                let x = self.v[x as usize] as usize;
                let y = self.v[y as usize] as usize;
                let height = n as usize;
                self.v[0xF] = 0;
                for yline in 0..height {
                    let pixel = self.memory[self.i as usize + yline];
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            //let index = x + xline + ((y + yline) * 64);
                            let index = (x + xline) % 64 + ((y + yline) % 32) * 64;

                            if self.screen[index] {
                                self.v[0xF] = 1;
                            }
                            self.screen[index] ^= true;
                        }
                    }
                }
                
            },
            0xE000 => {
                match nn {
                    0x9E => {
                        // Instrucción SKP: Saltar si presionada
                        if self.keypad[self.v[x as usize] as usize] {
                            self.pc += 2;
                        }
                    },
                    0xA1 => {
                        // Instrucción SKNP: Saltar si no presionada
                        if !self.keypad[self.v[x as usize] as usize] {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        // Instrucción no reconocida
                    }
                }   
            },
            0xF000 => {
                match nn {
                    0x07 => {
                        // Instrucción LD: Asignar
                        self.v[x as usize] = self.delay_timer;
                    },
                    0x0A => {
                        // Instrucción LD: Asignar
                        // Esperar a que se presione una tecla
                        self.v[x as usize] = 0;
                        for i in 0..16 {
                            if self.keypad[i] {
                                self.v[x as usize] = i as u8;
                                break;
                            }
                        }
                    },
                    0x15 => {
                        // Instrucción LD: Asignar
                        self.delay_timer = self.v[x as usize];
                    },
                    0x18 => {
                        // Instrucción LD: Asignar
                        self.sound_timer = self.v[x as usize];
                    },
                    0x1E => {
                        // Instrucción ADD: Sumar
                        self.i += self.v[x as usize] as u16;
                    },
                    0x29 => {
                        // Instrucción LD: Asignar
                        self.i = self.v[x as usize] as u16 * 5;
                    },
                    0x33 => {
                        // Instrucción BCD: Convertir a BCD
                        self.memory[self.i as usize] = self.v[x as usize] / 100;
                        self.memory[self.i as usize + 1] = (self.v[x as usize] / 10) % 10;
                        self.memory[self.i as usize + 2] = (self.v[x as usize] % 100) % 10;
                    },
                    0x55 => {
                        // Instrucción LD: Asignar
                        for i in 0..x + 1 {
                            self.memory[self.i as usize + i as usize] = self.v[i as usize];
                        }
                    },
                    0x65 => {
                        // Instrucción LD: Asignar
                        for i in 0..x + 1 {
                            self.v[i as usize] = self.memory[self.i as usize + i as usize];
                        }
                    },
                    _ => {
                        // Instrucción no reconocida
                    }
                }
            },
            _ => {
                // Instrucción no reconocida
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //instrucciones de prueba
    #[test]
    fn test_0x00eo() {
        let mut chip8 = super::Chip8::new();
        chip8.decode_and_execute_instruction(0x00E0);
        assert!(chip8.screen.iter().all(|&x| x == false));
    }

    #[test]
    fn test_0x00ee() {
        let mut chip8 = super::Chip8::new();
        chip8.sp = 1;
        chip8.stack[0] = 0x200;
        chip8.decode_and_execute_instruction(0x00EE);
        assert_eq!(chip8.sp, 0);
        assert_eq!(chip8.pc, 0x200);
    }

    #[test]
    fn test_0x1nnn() {
        let mut chip8 = super::Chip8::new();
        chip8.decode_and_execute_instruction(0x1234);
        assert_eq!(chip8.pc, 0x234);
    }

    #[test]
    fn test_0x2nnn() {
        let mut chip8 = super::Chip8::new();
        chip8.decode_and_execute_instruction(0x2345);
        assert_eq!(chip8.sp, 1);
        assert_eq!(chip8.stack[0], 0x200);
        assert_eq!(chip8.pc, 0x345);
    }

    #[test]
    fn test_0x3xnn() {
        let mut chip8 = super::Chip8::new();
        chip8.v[0] = 0x12;
        chip8.decode_and_execute_instruction(0x3012);
        assert_eq!(chip8.pc, 0x202);
        chip8.decode_and_execute_instruction(0x3013);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_0x4xnn() {
        let mut chip8 = super::Chip8::new();
        chip8.v[0] = 0x12;
        chip8.decode_and_execute_instruction(0x4012);
        assert_eq!(chip8.pc, 0x200);
        chip8.decode_and_execute_instruction(0x4013);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_annn() {
        let mut chip8 = super::Chip8::new();
        chip8.memory[0x200] = 0xA1;
        chip8.memory[0x201] = 0x23;
        chip8.emulate_cycle();
        
        assert_eq!(chip8.i, 0x123);
    }


}
