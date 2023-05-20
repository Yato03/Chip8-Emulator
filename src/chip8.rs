use std::fs;
use std::io;
use rand::Rng;

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
    keypad: [bool; 16],

    // Pantalla
    screen: [bool; 64 * 32],
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
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn load_rom(&mut self, rom_path: &str) -> Result<(), io::Error>  {
        // Aquí puedes implementar la lógica para cargar una ROM en la memoria del emulador
        // Puedes abrir el archivo ROM, leer su contenido y almacenarlo en la memoria adecuada
        // ...
        let rom = fs::read(rom_path)?;

        for (i, &byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }

        Ok(())
    }

    pub fn emulate_cycle(&mut self) {
        // Aquí puedes implementar el bucle de emulación principal del CHIP-8
        // Este bucle se encargará de buscar y ejecutar las instrucciones una tras otra
        // Puedes utilizar el contador de programa (PC) para obtener las instrucciones y actualizarlo adecuadamente
        // También necesitarás decodificar las instrucciones y ejecutar las operaciones correspondientes


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
                    },
                    _ => {
                        // Instrucción SYS: Salto a una dirección de memoria
                    }
                }
            },
            0x1000 => {
                // Instrucción JP: Salto a una dirección de memoria
                self.pc = nnn;
            },
            0x2000 => {
                // Instrucción CALL: Llamar a una subrutina
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
                self.v[x as usize] += nn as u8;
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
                        self.v[0xF] = self.v[x as usize] & 0x1;
                        self.v[x as usize] >>= 1;
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
                        self.v[x as usize] <<= 1;
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
                self.pc = nnn + self.v[0] as u16;
            },
            0xC000 => {
                // Instrucción RND: Generar número aleatorio
                self.v[x as usize] = rand::random::<u8>() & nn as u8;
            },
            /*0xD000 =>{
                // Instrucción DRW: Dibujar sprite
                
            },*/
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
                        // Instrucción LD: Asignar
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
