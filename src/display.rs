use std::fmt::format;

use minifb::{Window, WindowOptions};

const SCREEN_WIDTH: usize = 720;
const SCREEN_HEIGHT: usize = 360;
const BUFFER_WIDTH: usize = 64;
const BUFFER_HEIGHT: usize = 32;

pub struct Display {
    // Agrega los campos necesarios para administrar el estado de la pantalla
    window: Window,
    pixels: Vec<u32>,
}

impl Display {
    pub fn new(name: &str) -> Display {

        let window = Window::new(
            format!("CHIP-8 Emulator: {}", name).as_str(),
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions::default(),
        )
        .expect("Failed to create window");

        Display {
            window,
            pixels: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            // Inicializa otros campos necesarios
        }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn update(&mut self, buffer: &[bool; BUFFER_WIDTH * BUFFER_HEIGHT]) {
        // Calcular la escala para el escalado del búfer
        let scale_x = SCREEN_WIDTH / BUFFER_WIDTH;
        let scale_y = SCREEN_HEIGHT / BUFFER_HEIGHT;

        // Mapear los píxeles del búfer al nuevo tamaño de pantalla
        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let buffer_index = y * BUFFER_WIDTH + x;
                let pixel_value = buffer[buffer_index];
                let screen_x = x * scale_x;
                let screen_y = y * scale_y;

                // Asignar el valor del píxel al búfer de pantalla escalado
                for i in 0..scale_x {
                    for j in 0..scale_y {
                        let screen_index = (screen_y + j) * SCREEN_WIDTH + (screen_x + i);
                        self.pixels[screen_index] = if pixel_value { 0xFFFFFF } else { 0x000000 };
                    }
                }
            }
        }

        // Actualizar la ventana gráfica con el búfer convertido
        self.window
            .update_with_buffer(&self.pixels, SCREEN_WIDTH, SCREEN_HEIGHT)
            .expect("Failed to update window");
    }
}