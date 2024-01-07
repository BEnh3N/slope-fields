use crate::{draw_slope_line, pixel_to_grid_space, sigmoid, DY_DX, WIDTH};
use rayon::prelude::*;

// Holds the internal state of the program
pub struct World {
    pub mouse_pos: (f32, f32),
}

impl World {
    pub fn new() -> Self {
        Self {
            mouse_pos: (0., 0.),
        }
    }

    // fn update(&mut self) {}

    pub fn draw(&self, frame: &mut [u8]) {
        frame
            .par_chunks_exact_mut(4)
            .enumerate()
            .for_each(|(i, pixel)| {
                let x = (i % WIDTH as usize) as i32;
                let y = (i / WIDTH as usize) as i32;
                let (rx, ry) = pixel_to_grid_space(x, y);

                let m = DY_DX(rx, ry);

                let g = sigmoid(m);
                let mut rgba = [0, 0, 0, 0xff];
                if g > 0.0 {
                    rgba[0] = (g * 255.0) as u8
                } else {
                    rgba[2] = (-g * 255.0) as u8
                }

                pixel.copy_from_slice(&rgba);
            });

        // Go from x = [-9, 9] and y = [-9, 9] and draw slope lines
        for i in 0..=18 {
            for j in 0..=18 {
                let rx = (i - 9) as f32;
                let ry = (j - 9) as f32;
                draw_slope_line(frame, rx, ry, 40., true);
            }
        }

        // Draw the slope line at the current mouse position
        let x = self.mouse_pos.0 as i32;
        let y = self.mouse_pos.1 as i32;
        let (rx, ry) = pixel_to_grid_space(x, y);
        draw_slope_line(frame, rx, ry, 100., true);
    }
}
